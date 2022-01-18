#[derive(Debug)]
struct Config {
    pub width: u16,
    pub height: u16,
    pub aspect_ratio: f64,
    pub samples: u16,
    pub max_depth: u16,
    pub output: String,
}

impl Config {
    pub fn new(w: u16, h: u16, s: u16, md: u16, out: String) -> Config {
        Config {
            width: w,
            height: h,
            aspect_ratio: w as f64 / h as f64,
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
            aspect_ratio: 0.0,
            samples: 0,
            max_depth: 0,
            output: "",
            help: true,
        }
    }

    pub fn parse_args(args: &[str]) -> Result<Config, &str> {
        let w: u16;
        let h: u16;
        let ar: f64;
        let s: u16;
        let md: u16;
        let out: String;
        let output_set = false;

        if args.len() < 2 {
            return Err("must specify output filename");
        }

        let it = args.iter();
        let _ = it.next();
        let arg = it.next();
        while arg != None {
            match Some(arg) {
                arg == "-w" => {

                }
            }

        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(1920, 1080, 64, 32, "out.ppm")
    }
}
