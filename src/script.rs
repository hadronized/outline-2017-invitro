#[macro_export]
macro_rules! script_text {
  ($rasterizer:ident, $name:expr, $size:expr) => {
    $rasterizer.rasterize($name, $size).unwrap()
  }
}

// information
pub const SCRIPT_DSR_AND_TUS: &'static str = "DES!RE and The Undead Sceners";
pub const SCRIPT_PROUD_TO_PRESENT: &'static str = "are proud to present";
pub const SCRIPT_THAT_ATARI_PARTY: &'static str = "that Atari party where other platforms are welcome too will return";
pub const SCRIPT_FARM: &'static str = "to the former pauper colony farm called \"De Riethoeve\"";
pub const SCRIPT_LOCATION: &'static str = "Paasloregel 45 Willemsoord, The Netherlands";
pub const SCRIPT_DATE: &'static str = "Ascension day weekend, Thursday May 25th – Sunday May 28th of 2017";
pub const SCRIPT_COMES_WITH: &'static str = "it will come with";
pub const SCRIPT_FEATURES: &'static [&'static str] = &[
  "a spacious main hall with a large format projection screen",
  "a private campsite",
  "an indoor bar and lounge",
  "even a pond for the PMP duckies"
];
pub const SCRIPT_FEATURES_SUM_UP: &'static [&'static str] = &[
  "Outline offers a wild variety of live sets and live coding",
  "dj and vj sessions",
  "a demoshow",
  "sleeping facilities, affordable catering, beverage options",
];
pub const SCRIPT_EXPERIENCE_IT: &'static str = "experience it for yourself and find out";
pub const SCRIPT_SECRET_CHEESE: &'static str = "the secret behind all that Dutch cheese!";

// greets
pub const SCRIPT_GREETS: &'static [&'static str] = &[
  "Alcatraz",
  "Ate Bit",
  "Cocoon",
  "CRTC",
  "Ctrl-Alt-Test",
  "Darklite",
  "Deadliners",
  "Equinox",
  "Flush",
  "Focus Design",
  "Hornet",
  "Inque",
  "LNX",
  "Nah Kolor",
  "NinjaDev",
  "Orb",
  "Oxyron",
  "Planet Earth",
  "Poo Brain",
  "Punkfloyd",
  "Quebarium",
  "Rebels",
  "Resistance",
  "Sensenstahl",
  "sam",
  "Still",
  "SVatG",
  "Titan",
  "tmp",
  "TPB",
  "TRSi",
  "Vision",
  "XMEN",
];

// credits
pub const SCRIPT_CREDIT_CODE: &'static str      = "Code, dir  phaazon";
pub const SCRIPT_CREDIT_GFX_1: &'static str     = "Gfx          preej";
pub const SCRIPT_CREDIT_GFX_2: &'static str     = "Gfx          lycan";
pub const SCRIPT_CREDIT_GFX_3: &'static str     = "Gfx         mxbyte";
pub const SCRIPT_CREDIT_GFX_4: &'static str     = "Gfx          optic";
pub const SCRIPT_CREDIT_GFX_5: &'static str     = "Gfx        ploopie";
pub const SCRIPT_CREDIT_MUSIC: &'static str     = "Music      defcon8";
pub const SCRIPT_CREDIT_DIRECTION: &'static str = "Direction  ramonb5";
pub const SCRIPT_CREDIT_SUPPORT: &'static str   = "Support      havoc";
