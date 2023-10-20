use subprocess::Exec;
use roxmltree::{self, ParsingOptions};
use std::{path::PathBuf, io::{self, Write}, collections::HashMap};
use toml::{self, Table};
use std::fs;
use console::{Term, style};

static MAMEDEP_REVISION: i64 = 0;

fn depcache_load(mame: &str, dotdir: &PathBuf, _stdout: &Term) -> Result<Table, String> {

    let mdep = dotdir.join("machine.dep");
    let mamev = Exec::shell(format!(r#"{} -help"#, mame)).capture()
        .expect("!!").stdout_str();
    let binding = Iterator::collect::<Vec<_>>(mamev.split_terminator('\n').map(|s| s.to_owned()));
    let mamev2 = binding.first();
    let mut mamev3 = mamev2.unwrap().chars();
    mamev3.next_back();
    let mamever = mamev3.as_str().to_string();
    println!("Running on {}", style(mamever.clone()).bold().cyan());
    
    if !mdep.exists() {
        return Err(mamever);
    }
    
    let mdp = fs::read_to_string(mdep);
    if mdp.is_err() {
        panic!("Unable to read machine.dep.");
    };

    let mdp2 = mdp.unwrap();
    let mdp3 = mdp2.as_str();
    let depcache: Result<Table, toml::de::Error> = toml::from_str(mdp3);
    if depcache.is_err() {
        println!("machine.dep TOML is invalid");
        return Err(mamever);
    }

    let dc = depcache.unwrap();

    if dc.get("MDEP_VER").unwrap_or(&toml::Value::Integer(-1)).as_integer() != Some(MAMEDEP_REVISION) {
        println!("machine.dep TOML is for a different version of mame-dl2");
        return Err(mamever);
    }

    if dc.get("MAME_VER").unwrap_or(&toml::Value::String("---".to_string())).as_str() != Some(&mamever) {
        println!("MAME version doesn't match");
        return Err(mamever);
    }

    return Ok(dc);

}

pub fn depcache_init(mame: &str, dotdir: &PathBuf, stdout: &Term) -> Result<Table, bool> {

    let depcache = depcache_load(mame, dotdir, stdout);
    if !depcache.is_err() {
        return Ok(depcache.unwrap()); // leave if we loaded the cache
    }

    println!("{}\n",style(
r#"
  The dependency cache could not be loaded.
  Generating it will take about a minute.
  This should happen on your first use of the software,
  or when you update MAME.
  This could take up about 2GB of RAM, though!"#).yellow().on_black());

    println!("Capturing XML from 'mame -listxml'...");
    let mxml_s = Exec::shell(format!(r#"{} -listxml"#, mame)).capture().expect("!!").stdout_str();
    let mxml = mxml_s.as_str();
    println!("Parsing...");

    let mame_xml = roxmltree::Document::parse_with_options(mxml, ParsingOptions {
        allow_dtd: true, ..Default::default()
    }).expect("Couldn't parse MAME XML.");

    let gen_toml = depcache_generate_from_xml(&mame_xml, stdout, depcache.unwrap_err());

    fs::write(dotdir.join("machine.dep"), gen_toml).expect("Couldn't write machine dep TOML");

    return Ok(depcache_load(mame, dotdir, stdout).expect("Unable to parse the generated TOML."));

}

fn depcache_generate_from_xml(doc: &roxmltree::Document, stdout: &Term, mamever: String) -> String {
    println!("Generating dependency cache from XML:");
    let mut gen_toml = String::new();
    gen_toml.push_str(r#"MAME_VER=""#);
    gen_toml.push_str(mamever.as_str());
    gen_toml.push('"');
    gen_toml.push('\n');
    gen_toml.push_str("MDEP_VER=");
    gen_toml.push_str(MAMEDEP_REVISION.to_string().as_str());
    gen_toml.push('\n');
    let machines = doc.root_element().children().filter(|e| e.has_tag_name("machine"));
    let mut machine_index: HashMap<String,roxmltree::Node> = HashMap::new();
    for machine in machines {
        let name = machine.attribute("name").unwrap().to_string();
        machine_index.insert(name.clone(), machine);
    }
    // old MAME versions with XML call them <game>s instead of <machine>s
    let games = doc.root_element().children().filter(|e| e.has_tag_name("game"));
    for machine in games {
        let name = machine.attribute("name").unwrap().to_string();
        machine_index.insert(name.clone(), machine);
    }
    let m_count = machine_index.len();
    let mut m_in: usize = 0;
    for mi in &machine_index {
        let mut deps: Vec<String> = Vec::new();
        let name = mi.0;
        let machine = mi.1;
        stdout.clear_line().unwrap();
        print!("Resolving dependency {} / {}", m_in, m_count);
        io::stdout().flush().unwrap();
        dep_get(&machine, &machine_index, deps.as_mut());
        if deps.len() == 0 { // if no deps were generated don't write this machine to the file
            continue;
        }
        gen_toml.push_str(&name);
        gen_toml.push_str("=[");
        let mut first_element_complete = false;
        for d in deps {
            if first_element_complete { gen_toml.push(','); }
            gen_toml.push('"');
            gen_toml.push_str(d.as_str());
            gen_toml.push('"');
            first_element_complete = true;
        }
        gen_toml.push_str("]\n");
        m_in += 1;
    }
    print!("\n");
    return gen_toml;
}

fn dep_get(machine: &roxmltree::Node, machine_index: &HashMap<String,roxmltree::Node>, depslist: &mut Vec<String>) {
    let name = machine.attribute("name").unwrap();
    // Remove this for split romsets.
    if machine.has_attribute("cloneof") {
        let clone_of = machine.attribute("cloneof").unwrap();
        dep_get(machine_index.get(clone_of).unwrap(), machine_index, depslist);
        return;
    }
    if machine.children().filter(|e| e.has_tag_name("rom")).count() < 1 {
        return;
    }
    for device_ref in machine.children().filter(|e| e.has_tag_name("device_ref")) {
        let dr = device_ref.attribute("name").unwrap();
        dep_get(machine_index.get(dr).unwrap(), machine_index, depslist);
    }
    depslist.push(name.to_string());
}