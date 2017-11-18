
use regex::Regex;


//const DEFAULT_DISPATCH_REGEX: &'static str = "[^/]+";

#[derive(Debug)]
pub enum RData {
	static_path(String),
	dynamic_path(Vec<String>),
}

#[derive(Debug)]	
pub struct RouteParser{
	route_datas: Vec<RData>,
}


fn match_brks(route: &str) {
	let mut bytes = route.bytes();
	let mut match_num = 0;
	
	for byte in bytes {
		if b'[' == byte {
			match_num = match_num + 1;
		}
		
		if b']' == byte {
			if match_num > 2 {
				panic!("Number of opening '[' and closing ']' does not match");
			}
			
			match_num = match_num - 1;
		}	
	}
	
	if match_num != 0 {
		panic!("Number of opening '[' and closing ']' does not match");
	}
}

pub fn parse(route: &str)-> Vec<Vec<RouteParser>>{
	match_brks(route);
	let mut rslt: Vec<Vec<RouteParser>> = Vec::new();
	let re = Regex::new(r"\{\s* ([a-zA-Z_][a-zA-Z0-9_-]*) \s*(?: : \s* ([^{}]*))?\}|\[/").unwrap();
    
	let route_without_closing_options = route.trim_right_matches(']');
	if route.len() - 1 >  route_without_closing_options.len() {
		panic!("Found double closing options together instead of one");
	}

	let fields: Vec<&str> = re.splitn(route_without_closing_options,2).collect();
	let mut segment: String;
	let mut temp_vec: Vec<&str> = Vec::new();
	for field in fields.iter() {
		if field.is_empty() {
			continue;
		}
		temp_vec.push(field);
		segment = temp_vec.concat();
		let route_parser: Vec<RouteParser> = parse_place_holders(segment.as_ref());
		rslt.push(route_parser);
	}
	
	rslt
}
	
	
fn parse_place_holders(route: &str)->Vec<RouteParser> {
	let re = Regex::new(r"\{\s*([a-zA-Z_][a-zA-Z0-9_-]*)\s*(?::\s*([^{}]*))?\}").unwrap();
	let mut offset = 0;

	let mut rt_parser_vec: Vec<RouteParser> = Vec::new();
	for capture in re.captures_iter(route) {
		let mut lvec: Vec<String> = Vec::new();
		
        let (start, end) =  match capture.get(0){ 
			Some(l_match) => (l_match.start(), l_match.end()),
			None => panic!("Start and end should not be None"),
		};

		let ss: String  =  route.chars().skip(offset).take(start - offset).collect();
		offset = end;
		let static_path = RData::static_path(ss);
		lvec.push(capture[1].to_string());
		
		if capture.get(2) == None {
			lvec.push("[^/]+".to_string());
		} else {
			lvec.push(capture[2].to_string());
		}

		let dymanic_variable = RData::dynamic_path(lvec);
		let rt_parser = RouteParser {
			route_datas: vec!(static_path, dymanic_variable)
		};
		rt_parser_vec.push(rt_parser);
	}
	
	if offset != route.len() {
		let ss: String  =  route.chars().skip(offset).take(route.len() - offset).collect();

		let static_path = RData::static_path(ss);
		let rt_parser = RouteParser { route_datas: vec!(static_path)};
        rt_parser_vec.push(rt_parser);
    }
	
	rt_parser_vec
} 





#[cfg(test)]
mod tests {
	use::routerparser::{parse ,RouteParser };
    #[test]
    fn with_options() {
		let mvec: Vec<Vec<RouteParser>> = parse("/fixedRoutePart[/{varName}/moreFixed/{varName2:[0-9]+}]");
        assert_eq!(2 , mvec.len());
    }
    
        #[test]
    fn without_options() {
		let mvec: Vec<Vec<RouteParser>> = parse("/fixedRoutePart/");
        assert_eq!(1 , mvec.len());
    }
}
