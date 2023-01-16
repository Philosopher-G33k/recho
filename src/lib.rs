
pub struct Config {
    pub args: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments provided");
        }

        let mut contents: Vec<String> = vec![];
        let mut i = 0;
        for content in args {
            if i == 0 {
                i = i+1;
                continue;
            }
            contents.insert(contents.len(), content.clone());
        }
        
        Ok(Config { args: contents })
    }
}

pub fn run(config: Config) {
    let echo_string = config.args.join(" ");
    println!("{echo_string}");
}