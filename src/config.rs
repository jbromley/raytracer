#[derive(Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples: u16,
    pub max_depth: u16,
    pub output: String,
    pub help: bool,
}

impl Config {
    pub fn new(w: u32, h: u32, s: u16, md: u16, out: String) -> Config {
        Config {
            width: w,
            height: h,
            samples: s,
            max_depth: md,
            output: out,
            help: false,
        }
    }

    pub fn need_help() -> Config {
        Config {
            width: 0,
            height: 0,
            samples: 0,
            max_depth: 0,
            output: String::from(""),
            help: true,
        }
    }

    pub fn aspect_ratio(&self) -> Option<f64> {
        if self.height == 0 {
            return None;
        }

        return Some(self.width as f64 / self.height as f64);
    }

    pub fn parse_args(args: &Vec<String>) -> Result<Config, &str> {
        let mut it = args.iter();
        let mut cfg = Config::default();

        it.next();
        loop {
            let token = it.next();
            if token == None {
                break;
            }

            if let Some(arg) = token {
                if *arg == String::from("-w") {
                    if let Some(token) = it.next() {
                        match token.parse::<u32>() {
                            Ok(w) => cfg.width = w,
                            Err(_) => return Err("invalid width"),
                        }
                    } else {
                        return Err("no width provided");
                    }
                } else if *arg == String::from("-h") {
                    if let Some(token) = it.next() {
                        match token.parse::<u32>() {
                            Ok(h) => cfg.height = h,
                            Err(_) => return Err("invalid height"),
                        }
                    } else {
                        return Err("no height provided");
                    }
                } else if *arg == String::from("-s") {
                    if let Some(token) = it.next() {
                        match token.parse::<u16>() {
                            Ok(s) => cfg.samples = s,
                            Err(_) => return Err("invalid number of samples"),
                        }
                    } else {
                        return Err("no number of samples provided");
                    }
                } else if *arg == String::from("-d") {
                    if let Some(token) = it.next() {
                        match token.parse::<u16>() {
                            Ok(md) => cfg.max_depth = md,
                            Err(_) => return Err("invalid maximum depth"),
                        }
                    } else {
                        return Err("no maximum depth provided");
                    }
                } else if *arg == String::from("--help") {
                    return Ok(Config::need_help());
                } else {
                    cfg.output = arg.to_string();
                }
            }
        }

        if cfg.output.is_empty() {
            return Err("no output file specified");
        }
        return Ok(cfg);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(1920, 1080, 64, 32, String::from(""))
    }
}

#[cfg(test)]
use float_cmp::assert_approx_eq;

#[cfg(test)]
mod tests  {
    use super::*;

    #[test]
    fn test_aspect_ratio() {
        let cfg = Config::default();
        let expected: f64 = 1920.0 / 1080.0;
        if let Some(aspect_ratio) = cfg.aspect_ratio() {
            assert_approx_eq!(f64, aspect_ratio, expected);
        } else {
            panic!("None returned for valid aspect ratio");
        }

        let cfg = Config::need_help();
        assert_eq!(cfg.aspect_ratio(), None);
    }

    #[test]
    fn test_only_output() {
        let args: Vec<String> = vec![String::from("argparse"), String::from("output.ppm")];
        match Config::parse_args(&args) {
            Ok(cfg) => assert_eq!(cfg.output, String::from("output.ppm")),
            Err(e) => panic!("Error {} parsing no-option args", e),
        }
    }

    #[test]
    fn test_no_output() {
        let args: Vec<String> = vec![String::from("argparse")];
        println!("{:?}", Config::parse_args(&args));
        match Config::parse_args(&args) {
            Ok(_) => panic!("Valid Config from invalid args"),
            Err(e) => assert_eq!(e, "no output file specified"),
        }
    }

    #[test]
    fn test_valid_width_arg() {
        let args: Vec<String> = vec![String::from("argparse"),
                                     String::from("-w"), String::from("1680"),
                                     String::from("output.ppm")];
        match Config::parse_args(&args) {
            Ok(cfg) => {
                assert_eq!(cfg.width, 1680);
                assert_eq!(cfg.output, String::from("output.ppm"));
            },
            Err(_) => panic!("can't create config from `argparse -w 1680 output.ppm`"),
        }
    }

    #[test]
    fn test_invalid_width_arg() {
        let args: Vec<String> = vec![String::from("argparse"),
                                     String::from("-w"), String::from("x1680"),
                                     String::from("output.ppm")];
        match Config::parse_args(&args) {
            Ok(_) => panic!("valid config from `argparse -w x1680 output.ppm"),
            Err(s) => assert_eq!(s, "invalid width"),
        }
    }

    #[test]
    fn test_valid_width_no_output_arg() {
        let args: Vec<String> = vec![String::from("argparse"),
                                     String::from("-w"), String::from("1680")];
        match Config::parse_args(&args) {
            Ok(_) => panic!("valid Config without output file"),
            Err(e) => assert_eq!(e, "no output file specified"),
        }
    }

    #[test]
    fn test_all() {
        let args: Vec<String> = "argparse -w 640 -h 480 -s 64 -d 16 output.ppm"
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();
        let cfg = Config::parse_args(&args);
        match cfg {
            Ok(c) => {
                assert_eq!(c.width, 640);
                assert_eq!(c.height, 480);
                assert_eq!(c.samples, 64);
                assert_eq!(c.max_depth, 16);
                assert_eq!(c.output, String::from("output.ppm"));
            },
            Err(_) => panic!("error from valid arguments"),
        }
    }
}
