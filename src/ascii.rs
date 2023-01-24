use regex::Regex;
use crate::color::*;
use std::collections::HashMap;
use sysinfo::System;
use serde_json::to_string;
use crate::utils::verify_os;
pub fn ascii_storage() -> HashMap<String, String> {
    let mut ascii_art: HashMap<String, String> = HashMap::new();
    ascii_art.insert("custom".to_string(), r#"
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
        "#.to_string());
    ascii_art.insert("openSUSE Tumbleweed".to_string(), r#"
    {c3}                                   ......
    {c3}    .,cdxxxoc,.               .:kKMMMNWMMMNk:.
    {c3}  cKMMN0OOOKWMMXo. ;        ;0MWk:.      .:OMMk.
   {c3} ;WMK;.       .lKMMNM,     :NMK,             .OMW;
  {c3} cMW;            'WMMMN   ,XMK,                 oMM'
 {c3} .MMc               ..;l. xMN:                    KM0
 {c3} 'MM.                   'NMO                      oMM
 {c3} .MM,                 .kMMl                       xMN
  {c3} KM0               .kMM0. .dl:,..               .WMd
  {c3} .XM0.           ,OMMK,    OMMMK.              .XMK
    {c3} oWMO:.    .;xNMMk,       NNNMKl.          .xWMx
      {c3} :ONMMNXMMMKx;          .  ,xNMWKkxllox0NMWk,
          {c3} .....                    .:dOOXXKOxl,
    "#.to_string());
    ascii_art.insert("Arch Linux".to_string(), r#"
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
/+. "#.to_string());
    ascii_art.insert("Pop!_OS".to_string(), r#"
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
    "#.to_string());
    ascii_art.insert("Ubuntu".to_string(), r#"
{c4}              .-/+oossssoo+\-.
{c4}          ´:+ssssssssssssssssss+:`
{c4}        -+ssssssssssssssssssyyssss+-
{c4}      .ossssssssssssssssss{c8}dMMMNy{c4}sssso.
{c4}     /sssssssssss{c8}hdmmNNmmyNMMMMh{c4}ssssss\
{c4}    +sssssssss{c8}hm{c4}yd{c8}MMMMMMMNddddy{c4}ssssssss+
{c4}   /ssssssss{c8}hNMMM{c4}yh{c8}hyyyyhmNMMMNh{c4}ssssssss\
{c4}  .ssssssss{c8}dMMMNh{c4}ssssssssss{c8}hNMMMd{c4}ssssssss.
{c4}  +ssss{c8}hhhyNMMNy{c4}ssssssssssss{c8}yNMMMy{c4}sssssss+
{c4}  oss{c8}yNMMMNyMMh{c4}ssssssssssssss{c8}hmmmh{c4}ssssssso
{c4}  oss{c8}yNMMMNyMMh{c4}sssssssssssssshmmmh{c4}ssssssso
{c4}  +ssss{c8}hhhyNMMNy{c4}ssssssssssss{c8}yNMMMy{c4}sssssss+
{c4}  .ssssssss{c8}dMMMNh{c4}ssssssssss{c8}hNMMMd{c4}ssssssss.
{c4}   \ssssssss{c8}hNMMM{c4}yh{c8}hyyyyhdNMMMNh{c4}ssssssss/
{c4}    +sssssssss{c8}dm{c4}yd{c8}MMMMMMMMddddy{c4}ssssssss+
{c4}     \sssssssssss{c8}hdmNNNNmyNMMMMh{c4}ssssss/
{c4}      .ossssssssssssssssss{c8}dMMMNy{c4}sssso.
{c4}        -+sssssssssssssssss{c8}yyy{c4}ssss+-
{c4}          `:+ssssssssssssssssss+:`
{c4}              .-\+oossssoo+/-.
        "#.to_string(),
    );
    ascii_art.insert("Fedora".to_string(), r#"
    {c14}         .',;::::;,'.
         .';:cccccccccccc:;,.
      .;cccccccccccccccccccccc;.
    .:cccccccccccccccccccccccccc:.
  .;ccccccccccccc;{c8}.:dddl:.{c14};ccccccc;.
 .:ccccccccccccc;{c8}OWMKOOXMWd{c14};ccccccc:.
.:ccccccccccccc;{c8}KMMc{c14};cc;{c8}xMMc{c14};ccccccc:.
,cccccccccccccc;{c8}MMM.{c14};cc;{c8};WW:{c14};cccccccc,
:cccccccccccccc;{c8}MMM.{c14};cccccccccccccccc:
:ccccccc;{c8}oxOOOo{c14};{c8}MMM0OOk.{c14};cccccccccccc:
cccccc;{c8}0MMKxdd:{c14};{c8}MMMkddc.{c14};cccccccccccc;
ccccc;{c8}XM0'{c14};cccc;{c8}MMM.{c14};cccccccccccccccc'
ccccc;{c8}MMo{c14};ccccc;{c8}MMW.{c14};ccccccccccccccc;
ccccc;{c8}0MNc.{c14}ccc{c8}.xMMd{c14};ccccccccccccccc;
cccccc;{c8}dNMWXXXWM0:{c14};cccccccccccccc:,
cccccccc;{c8}.:odl:.{c14};cccccccccccccc:,.
:cccccccccccccccccccccccccccc:'.
.:cccccccccccccccccccccc:;,..
  '::cccccccccccccc::;,.
    {reset}"#.to_string());
    ascii_art.insert("Manjaro".to_string(), r#"{c3}
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
    "#.to_string());
    ascii_art.insert("EndeavourOS".to_string(), r#"
{c2}                     ./{c6}o{c4}.
{c2}                   ./{c6}sssso{c4}-
{c2}                 `:{c6}osssssss+{c4}-
{c2}               `:+{c6}sssssssssso{c4}/.
{c2}             `-/o{c6}ssssssssssssso{c4}/.
{c2}           `-/+{c6}sssssssssssssssso{c4}+:`
{c2}         `-:/+{c6}sssssssssssssssssso{c4}+/.
{c2}       `.://o{c6}sssssssssssssssssssso{c4}++-
{c2}      .://+{c6}ssssssssssssssssssssssso{c4}++:
{c2}    .:///o{c6}ssssssssssssssssssssssssso{c4}++:
{c2}  `:////{c6}ssssssssssssssssssssssssssso{c4}+++.
{c2}`-////+{c6}ssssssssssssssssssssssssssso{c4}++++-
{c2} `..-+{c6}oosssssssssssssssssssssssso{c4}+++++/`
   ./++++++++++++++++++++++++++++++/:.
  `:::::::::::::::::::::::::------``
    "#.to_string());

    ascii_art.insert("Windows".to_string(), r#"
 {c7}
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################

################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
################  ################
"#.to_string());
    //let json_string = to_string(&ascii_art).unwrap();
   //let my_art = ascii_art.get(verify_os(&system).as_str()).unwrap();
    let mut my_art ="";

    for (key, value) in ascii_art.iter() {

        println!("{}",ascii_art.get(key).unwrap())
    }
    return  ascii_art;



}
pub fn find_art(){

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
