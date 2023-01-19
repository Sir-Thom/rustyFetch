use regex::Regex;
use crate::color::*;
use std::collections::HashMap;

pub fn ascii_storerage(){
    let mut ascii_art: HashMap<&str, &str> = HashMap::new();
    ascii_art.insert("custom", r#"
{c2}   ///
{c2}   -::-`:     {c3}.--/:-
{c2}   `-:::::..::{c3}+///+//++:+
{c2}      +-:::::-:{c3}/++++++++++//:--
{c2}       -:::::--{c3}/++++++++++++++//:
{c2}       .-:::::::{c3}//++++++++++++++//+:
{c2}       .---::::::{c3}//////++++++++++++/+
{c3}  ///:/++++++++++++++////////+++++++//::.
{c3} +/++++++++++++++++++++++++++///////++//--
{c3} ://///////////////////////////++////////:.
{c3}     /..-://////////////////++/////////////:.
{c14}     `:+{c6}oooooooooooo::+oooooooo:-{c3}/++++++:o//
{c14}    `-+{c6}oooooooooooo+  /oooooooo` -ooooos{c14}+-
{c14}    -+++{c6}ooooooooooo+  /oooooooo` -ooooooso{c14}.
{c14}    -+++{c6}ooooooooooo+--+oooooooo--/ooooooso{c14}.
{c14}  -:++++{c6}oooooooosysoooooooooooooosyyoooooo{c14}/.
{c14}  `-o+++{c6}ooooooooossooooooooooooooossoooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  `-o+++{c6}oooooooooooooooooooooooooooooooooo{c14}+-
{c14}  .:+++++++{c6}ooooooooooooooooooooooooooooo{c14}++/.
{c14}    .://+++++++++++++++{c6}oo+oooooooooo{c14}+++/::.`
          {c14}///////////////////////////:
        "#);
    ascii_art.insert("openSUSE Tumbleweed",r#"
    {c3}                                     ......
     .,cdxxxoc,.               .:kKMMMNWMMMNk:.
    cKMMN0OOOKWMMXo. ;        ;0MWk:.      .:OMMk.
  ;WMK;.       .lKMMNM,     :NMK,             .OMW;
 cMW;            'WMMMN   ,XMK,                 oMM'
.MMc               ..;l. xMN:                    KM0
'MM.                   'NMO                      oMM
.MM,                 .kMMl                       xMN
 KM0               .kMM0. .dl:,..               .WMd
 .XM0.           ,OMMK,    OMMMK.              .XMK
   oWMO:.    .;xNMMk,       NNNMKl.          .xWMx
     :ONMMNXMMMKx;          .  ,xNMWKkxllox0NMWk,
         .....                    .:dOOXXKOxl,
    "#);
    ascii_art.insert("Arch Linux", r#"
    {c5}                    y:
                  sMN-
                 +MMMm`
                /MMMMMd`
               :NMMMMMMy
              -NMMMMMMMMs
             .NMMMMMMMMMM+
            .mMMMMMMMMMMMM+
            oNMMMMMMMMMMMMM+
          `+:-+NMMMMMMMMMMMM+
          .sNMNhNMMMMMMMMMMMM/
        `hho/sNMMMMMMMMMMMMMMM/
       `.`omMMmMMMMMMMMMMMMMMMM+
      .mMNdshMMMMd+::oNMMMMMMMMMo
     .mMMMMMMMMM+     `yMMMMMMMMMs
    .NMMMMMMMMM/        yMMMMMMMMMy
   -NMMMMMMMMMh         `mNMMMMMMMMd`
  /NMMMNds+:.`             `-/oymMMMm.
 +Mmy/.                          `:smN:
/+. "#);
    ascii_art.insert("Pop!_OS",r#"
    {c5}         /////////////
         /////////////////////
      ///////{c8}*767{c5}////////////////
    //////{c8}7676767676*{c5}//////////////
   /////{c8}76767{c5}//{c8}7676767{c5}//////////////
  /////{c8}767676{c5}///{c8}*76767{c5}///////////////
 ///////{c8}767676{c5}///{c8}76767{c5}.///{c8}7676*{c5}///////
/////////{c8}767676{c5}//{c8}76767{c5}///{c8}767676{c5}////////
//////////{c8}76767676767{c5}////{c8}76767{c5}/////////
///////////{c8}76767676{c5}//////{c8}7676{c5}//////////
////////////,{c8}7676{c5},///////{c8}767{c5}///////////
/////////////*{c8}7676{c5}///////{c8}76{c5}////////////
///////////////{c8}7676{c5}////////////////////
 ///////////////{c8}7676{c5}///{c8}767{c5}////////////
  //////////////////////{c8}'{c5}////////////
   //////{c8}.7676767676767676767,{c5}//////
    /////{c8}767676767676767676767{c5}/////
      ///////////////////////////
         /////////////////////
             /////////////
    "#);

    let my_art = ascii_art.get("Pop!_OS").unwrap();

    println!("{}", translate_ascii_colors(my_art));
}
pub fn translate_ascii_colors(ascii: &str) -> String {
    let ascii = ascii
        .replace("{c1}", BLACK)
        .replace("{c2}", RED)
        .replace("{c3}", GREEN)
        .replace("{c4}", YELLOW)
        .replace("{c5}", BLUE)
        .replace("{c6}", MAGENTA)
        .replace("{c7}", CYAN)
        .replace("{c8}", WHITE)
        .replace("{c9}", BLACK_BRIGHT)
        .replace("{c10}", RED_BRIGHT)
        .replace("{c11}", GREEN_BRIGHT)
        .replace("{c12}", YELLOW_BRIGHT)
        .replace("{c13}", BLUE_BRIGHT)
        .replace("{c14}", MAGENTA_BRIGHT)
        .replace("{c15}", CYAN_BRIGHT)
        .replace("{c16}", WHITE_BRIGHT)
        .replace("{reset}", RESET);

    ascii
}
struct AsciiArt {
    name: String,
    art: String,
}
