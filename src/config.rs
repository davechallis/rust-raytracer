use std::path::Path;

use clap::{Arg, App};

pub struct Config {
    width: u32,
    height: u32,
    samples: u32,
    output: String,
    inline: bool,
}

impl<'a> Config {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn samples(&self) -> u32 {
        self.samples
    }

    pub fn output(&'a self) -> &'a Path {
        Path::new(&self.output)
    }

    pub fn inline(&self) -> bool {
        self.inline
    }

    pub fn from_cli_args() -> Self {
        let matches = App::new("raytracer")
            .arg(Arg::with_name("width")
               .long("width")
               .value_name("W")
               .help("Set width of generated image")
               .takes_value(true))
            .arg(Arg::with_name("height")
               .long("height")
               .value_name("H")
               .help("Set height of generated image")
               .takes_value(true))
            .arg(Arg::with_name("samples")
               .long("samples")
               .value_name("N")
               .help("Set number of samples per pixel")
               .takes_value(true))
            .arg(Arg::with_name("output")
               .long("output")
               .value_name("OUTPUT")
               .help("Set output path of generated image")
               .takes_value(true))
            .arg(Arg::with_name("inline")
               .long("inline")
               .help("Output image inline (for use with iTerm2)"))
        .get_matches();

        let width = matches.value_of("width").unwrap_or("200").parse().unwrap();
        let height = matches.value_of("height").unwrap_or("100").parse().unwrap();
        let samples = matches.value_of("samples").unwrap_or("10").parse().unwrap();
        let output = matches.value_of("output").unwrap_or("./raytracer.png").to_owned();
        let inline = matches.occurrences_of("inline") > 0;

        Self { width, height, samples, output, inline }
    }
}
