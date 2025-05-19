use std::fs;

use crate::http::{Methods, Response};

const FIRST_105_POKEMON_JSON: &str = r#"["bulbasaur","ivysaur","venusaur","charmander","charmeleon","charizard","squirtle","wartortle","blastoise","caterpie","metapod","butterfree","weedle","kakuna","beedrill","pidgey","pidgeotto","pidgeot","rattata","raticate","spearow","fearow","ekans","arbok","pikachu","raichu","sandshrew","sandslash","nidoran-f","nidorina","nidoqueen","nidoran-m","nidorino","nidoking","clefairy","clefable","vulpix","ninetales","jigglypuff","wigglytuff","zubat","golbat","oddish","gloom","vileplume","paras","parasect","venonat","venomoth","diglett","dugtrio","meowth","persian","psyduck","golduck","mankey","primeape","growlithe","arcanine","poliwag","poliwhirl","poliwrath","abra","kadabra","alakazam","machop","machoke","machamp","bellsprout","weepinbell","victreebel","tentacool","tentacruel","geodude","graveler","golem","ponyta","rapidash","slowpoke","slowbro","magnemite","magneton","farfetchd","doduo","dodrio","seel","dewgong","grimer","muk","shellder","cloyster","gastly","haunter","gengar","onix","drowzee","hypno","krabby","kingler","voltorb","electrode","exeggcute","exeggutor","cubone","marowak"]"#;

use super::server::Handler;
pub struct WebsiteHandler {
    public_folder_path: String,
}

impl WebsiteHandler {
    pub fn new(public_folder_path: String) -> Self {
        Self { public_folder_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_folder_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_folder_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Someone tried a Traversal attack! path {}", path.display());
                    None
                }
            }
            Err(_) => None
        }

    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Methods::GET => match request.path() {
                "/" => Response::new(crate::http::StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some("<h1> Hello world!</h1>".to_string()),
                ),
                "/pokemons" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some(FIRST_105_POKEMON_JSON.to_string()),
                ),
                path => match self.read_file(path) {
                    Some(content) => Response::new(crate::http::StatusCode::Ok, Some(content)),
                    None => Response::new(crate::http::StatusCode::Notfound, None),
                },
            },
            _ => Response::new(
                crate::http::StatusCode::Ok,
                Some("<h1> Hello! </h1>".to_string()),
            ),
        }
    }
}
