use std::fmt::Display;

// I'm taking this ascii art directly from /usr/bin/neofetch, roughly lines 5000 to 1000
// got bored.
// would be nice to have a cheeky cli tool which just draws these
// have been stripping color information
// need a nicer way of describing varitations of icons available for distros.

pub enum Distro {
    Aix,
    Alpine,
    AlpineSmall,
    Alter,
    Amazon,
    Anarchy,
    Android,
    AndroidSmall,
    Antergos,
    AntiX,
    AoscOs,
    AoscOsRetro,
    Apricity,
    Arcolinux,
    ArcolinuxSmall,
    Arch,
    ArchSmall,
    ArchOld,
    ArchBox,
    ArchLabs,
    ArchStrike,
    ArchMerge,
    ArtixSmall,
    Artix,
    Arya,
    Bedrock,
    Bitrig,
    Hash,
    XFerience,
}

impl Distro {
    pub fn ascii_art(&self) -> String {
        match self {
            Distro::Aix => ASCII_ART_AIX.to_string(),
            Distro::Alpine => ASCII_ART_ALPINE.to_string(),
            Distro::AlpineSmall => ASCII_ART_ALPINE_SMALL.to_string(),
            Distro::Alter => ASCII_ART_ALTER.to_string(),
            Distro::Amazon => ASCII_ART_AMAZON.to_string(),
            Distro::Anarchy => ASCII_ART_ANARCHY.to_string(),
            Distro::Android => ASCII_ART_ANDROID.to_string(),
            Distro::AndroidSmall => ASCII_ART_ANDROID_SMALL.to_string(),
            Distro::Antergos => ASCII_ART_ANTERGOS.to_string(),
            Distro::AntiX => ASCII_ART_ANTI_X.to_string(),
            Distro::AoscOs => ASCII_ART_AOSC_OS.to_string(),
            Distro::AoscOsRetro => ASCII_ART_AOSC_OS_RETRO.to_string(),
            Distro::Apricity => ASCII_ART_APRICITY.to_string(),
            Distro::Arcolinux => ASCII_ART_ARCOLINUX.to_string(),
            Distro::ArcolinuxSmall => ASCII_ART_ARCOLINUX_SMALL.to_string(),
            Distro::Arch => ASCII_ART_ARCH.to_string(),
            Distro::ArchSmall => ASCII_ART_ARCH_SMALL.to_string(),
            Distro::ArchOld => ASCII_ART_ARCH_OLD.to_string(),
            Distro::ArchBox => ASCII_ART_ARCH_BOX.to_string(),
            Distro::ArchLabs => ASCII_ART_ARCH_LABS.to_string(),
            Distro::ArchMerge => ASCII_ART_ARCH_MERGE.to_string(),
            Distro::ArchStrike => ASCII_ART_ARCH_STRIKE.to_string(),
            Distro::ArtixSmall => ASCII_ART_ARTIX_SMALL.to_string(),
            Distro::Artix => ASCII_ART_ARTIX.to_string(),
            Distro::Arya => ASCII_ART_ARYA.to_string(),
            Distro::Bedrock => ASCII_ART_BEDROCK.to_string(),
            Distro::Bitrig => ASCII_ART_BITRIG.to_string(),
            Distro::Hash => ASCII_ART_HASH.to_string(),
            Distro::XFerience => ASCII_ART_X_FERIENCE.to_string(),
        }
    }
}

impl Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Distro::Aix => "AIX",
            Distro::Alpine => "Alpine",
            Distro::AlpineSmall => "AlpineSmall",
            Distro::Alter => "Alter",
            Distro::Amazon => "Amazon",
            Distro::Anarchy => "Anarchy",
            Distro::Android => "Android",
            Distro::AndroidSmall => "AndroidSmall",
            Distro::Antergos => "Antergos",
            Distro::AntiX => "antiX",
            Distro::AoscOs => "AOSC OS",
            Distro::AoscOsRetro => "AOSC OS/Retro",
            Distro::Apricity => "Apricity",
            Distro::Arcolinux => "ArcoLinux",
            Distro::ArcolinuxSmall => "ArcoLinuxSmall",
            Distro::Arch => "Arch",
            Distro::ArchSmall => "Arch Small",
            Distro::ArchOld => "Arch Old",
            Distro::ArchBox => "Arch Box",
            Distro::ArchLabs => "Arch Labs",
            Distro::ArchMerge => "Arch Merge",
            Distro::ArchStrike => "Arch Strike",
            Distro::ArtixSmall => "Artix Small",
            Distro::Artix => "Artix",
            Distro::Arya => "Arya",
            Distro::Bedrock => "Bedrock",
            Distro::Bitrig => "Bitrig",
            Distro::Hash => "Hash",
            Distro::XFerience => "XFerience",
        };
        write!(f, "{}", msg)
    }
}

static ASCII_ART_AIX: &str = "
           `:+ssssossossss+-`
        .oys///oyhddddhyo///sy+.
      /yo:+hNNNNNNNNNNNNNNNNh+:oy/
    :h/:yNNNNNNNNNNNNNNNNNNNNNNy-+h:
  `ys.yNNNNNNNNNNNNNNNNNNNNNNNNNNy.ys
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oh
 h+-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN.oy
/d`mNNNNNNN/::mNNNd::m+:/dNNNo::dNNNd`m:
h//NNNNNNN: . .NNNh  mNo  od. -dNNNNN:+y
N.sNNNNNN+ -N/ -NNh  mNNd.   sNNNNNNNo-m
N.sNNNNNs  +oo  /Nh  mNNs` ` /mNNNNNNo-m
h//NNNNh  ossss` +h  md- .hm/ `sNNNNN:+y
:d`mNNN+/yNNNNNd//y//h//oNNNNy//sNNNd`m-
 yo-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNm.ss
 `h+-mNNNNNNNNNNNNNNNNNNNNNNNNNNNNm-oy
   sy.yNNNNNNNNNNNNNNNNNNNNNNNNNNs.yo
    :h+-yNNNNNNNNNNNNNNNNNNNNNNs-oh-
      :ys:/yNNNNNNNNNNNNNNNmy/:sy:
        .+ys///osyhhhhys+///sy+.
            -/osssossossso/-
";

static ASCII_ART_ALPINE: &str = "
       .hddddddddddddddddddddddh.
      :dddddddddddddddddddddddddd:
     /dddddddddddddddddddddddddddd/
    +dddddddddddddddddddddddddddddd+
  `sdddddddddddddddddddddddddddddddds`
 `ydddddddddddd++hdddddddddddddddddddy`
.hddddddddddd+`  `+ddddh:-sdddddddddddh.
hdddddddddd+`      `+y:    .sddddddddddh
ddddddddh+`   `//`   `.`     -sddddddddd
ddddddh+`   `/hddh/`   `:s-    -sddddddd
ddddh+`   `/+/dddddh/`   `+s-    -sddddd
ddd+`   `/o` :dddddddh/`   `oy-    .yddd
hdddyo+ohddyosdddddddddho+oydddy++ohdddh
.hddddddddddddddddddddddddddddddddddddh.
 `yddddddddddddddddddddddddddddddddddy`
  `sdddddddddddddddddddddddddddddddds`
    +dddddddddddddddddddddddddddddd+
     /dddddddddddddddddddddddddddd/
      :dddddddddddddddddddddddddd:
       .hddddddddddddddddddddddh.

";

static ASCII_ART_ALPINE_SMALL: &str = "
   /\\ /\\
  // \\  \\
 //   \\  \\
///    \\  \\
//      \\  \\
         \\
";

static ASCII_ART_ALTER: &str = "
                      %,
                    ^WWWw
                   'wwwwww
                  !wwwwwwww
                 #`wwwwwwwww
                @wwwwwwwwwwww
               wwwwwwwwwwwwwww
              wwwwwwwwwwwwwwwww
             wwwwwwwwwwwwwwwwwww
            wwwwwwwwwwwwwwwwwwww,
           w~1i.wwwwwwwwwwwwwwwww,
         3~:~1lli.wwwwwwwwwwwwwwww.
        :~~:~?ttttzwwwwwwwwwwwwwwww
       #<~:~~~~?llllltO-.wwwwwwwwwww
      #~:~~:~:~~?ltlltlttO-.wwwwwwwww
     @~:~~:~:~:~~(zttlltltlOda.wwwwwww
    @~:~~: ~:~~:~:(zltlltlO    a,wwwwww
   8~~:~~:~~~~:~~~~_1ltltu          ,www
  5~~:~~:~~:~~:~~:~~~_1ltq             N,,
 g~:~~:~~~:~~:~~:~:~~~~1q                N,
";

static ASCII_ART_AMAZON: &str = "
             `-/oydNNdyo:.`
      `.:+shmMMMMMMMMMMMMMMmhs+:.`
    -+hNNMMMMMMMMMMMMMMMMMMMMMMNNho-
.``      -/+shmNNMMMMMMNNmhs+/-      ``.
dNmhs+:.       `.:/oo/:.`       .:+shmNd
dMMMMMMMNdhs+:..        ..:+shdNMMMMMMMd
dMMMMMMMMMMMMMMNds    odNMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
dMMMMMMMMMMMMMMMMh    yMMMMMMMMMMMMMMMMd
.:+ydNMMMMMMMMMMMh    yMMMMMMMMMMMNdy+:.
     `.:+shNMMMMMh    yMMMMMNhs+:``
            `-+shy    shs+:`
";

static ASCII_ART_ANARCHY: &str = "
                         ..
                        ..
                      :..
                    :+++.
              .:::+++++++::.
          .:+######++++######+:.
       .+#########+++++##########:.
     .+##########+++++++##+#########+.
    +###########+++++++++############:
   +##########++++++#++++#+###########+
  +###########+++++###++++#+###########+
 :##########+#++++####++++#+############:
 ###########+++++#####+++++#+###++######+
.##########++++++#####++++++++++++#######.
.##########+++++++++++++++++++###########.
 #####++++++++++++++###++++++++#########+
 :###++++++++++#########+++++++#########:
  +######+++++##########++++++++#######+
   +####+++++###########+++++++++#####+
    :##++++++############++++++++++##:
     .++++++#############+++++++++++.
      :++++###############+++++++::
     .++. .:+##############+++++++..
     .:.      ..::++++++::..:+++++.
     .                       .:+++.
                                .::
                                   ..
                                    ..
";

static ASCII_ART_ANDROID_SMALL: &str = "
  ;,           ,;
   ';,.-----.,;'
  ,'           ',
 /    O     O    \\
|                 |
'-----------------'
";

static ASCII_ART_ANDROID: &str = "
         -o          o-
          +hydNNNNdyh+
        +mMMMMMMMMMMMMm+
      `dMMm:NMMMMMMN:mMMd`
      hMMMMMMMMMMMMMMMMMMh
  ..  yyyyyyyyyyyyyyyyyyyy  ..
.mMMm`MMMMMMMMMMMMMMMMMMMM`mMMm.
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
:MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM:
-MMMM-MMMMMMMMMMMMMMMMMMMM-MMMM-
 +yy+ MMMMMMMMMMMMMMMMMMMM +yy+
      mMMMMMMMMMMMMMMMMMMm
      `/++MMMMh++hMMMM++/`
          MMMMo  oMMMM
          MMMMo  oMMMM
          oNMm-  -mMNs
";

static ASCII_ART_ANTERGOS: &str = "
              `.-/::/-``
            .-/osssssssso/.
           :osyysssssssyyys+-
        `.+yyyysssssssssyyyyy+.
       `/syyyyyssssssssssyyyyys-`
      `/yhyyyyysss++ssosyyyyhhy/`
     .ohhhyyyyso++/+oso+syy+shhhho.
    .shhhhysoo++//+sss+++yyy+shhhhs.
   -yhhhhs+++++++ossso+++yyys+ohhddy:
  -yddhhyo+++++osyyss++++yyyyooyhdddy-
 .yddddhso++osyyyyys+++++yyhhsoshddddy`
`odddddhyosyhyyyyyy++++++yhhhyosddddddo
.dmdddddhhhhhhhyyyo+++++shhhhhohddddmmh.
ddmmdddddhhhhhhhso++++++yhhhhhhdddddmmdy
dmmmdddddddhhhyso++++++shhhhhddddddmmmmh
-dmmmdddddddhhyso++++oshhhhdddddddmmmmd-
.smmmmddddddddhhhhhhhhhdddddddddmmmms.
   `+ydmmmdddddddddddddddddddmmmmdy/.
      `.:+ooyyddddddddddddyyso+:.`
";

static ASCII_ART_ANTI_X: &str = "
                    \\
         , - ~ ^ ~ - \\        /
     , '              \\ ' ,  /
   ,                   \\   '/
  ,                     \\  / ,
 ,___,                   \\/   ,
 /   |   _  _  _|_ o     /\\   ,
|,   |  / |/ |  |  |    /  \\  ,
 \\,_/\\_/  |  |_/|_/|_/_/    \\,
   ,                  /     ,\\
     ,               /  , '   \\
      ' - , _ _ _ ,  '
";

static ASCII_ART_AOSC_OS_RETRO: &str = "
          .........
     ...................
   .....................################
 ..............     ....################
..............       ...################
.............         ..****************
............     .     .****************
...........     ...     ................
..........     .....     ...............
.........     .......     ...
 .......                   .
  .....      .........    ...........
  ....      .......       ...........
  ...      .......        ...........
  ................        ***********
  ................        ###########
  ****************
  ################
";

static ASCII_ART_AOSC_OS: &str = "
             .:+syhhhhys+:.
         .ohNMMMMMMMMMMMMMMNho.
      `+mMMMMMMMMMMmdmNMMMMMMMMm+`
     +NMMMMMMMMMMMM/   `./smMMMMMN+
   .mMMMMMMMMMMMMMMo        -yMMMMMm.
  :NMMMMMMMMMMMMMMMs          .hMMMMN:
 .NMMMMhmMMMMMMMMMMm+/-         oMMMMN.
 dMMMMs  ./ymMMMMMMMMMMNy.       sMMMMd
-MMMMN`      oMMMMMMMMMMMN:      `NMMMM-
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
/MMMMh       NMMMMMMMMMMMMm       hMMMM/
-MMMMN`      :MMMMMMMMMMMMy.     `NMMMM-
 dMMMMs       .yNMMMMMMMMMMMNy/. sMMMMd
 .NMMMMo         -/+sMMMMMMMMMMMmMMMMN.
  :NMMMMh.          .MMMMMMMMMMMMMMMN:
   .mMMMMMy-         NMMMMMMMMMMMMMm.
     +NMMMMMms/.`    mMMMMMMMMMMMN+
      `+mMMMMMMMMNmddMMMMMMMMMMm+`
         .ohNMMMMMMMMMMMMMMNho.
             .:+syhhhhys+:.
";

static ASCII_ART_APRICITY: &str = "
                                    ./o-
          ``...``              `:. -/:
     `-+ymNMMMMMNmho-`      :sdNNm/
   `+dMMMMMMMMMMMMMMMmo` sh:.:::-
  /mMMMMMMMMMMMMMMMMMMMm/`sNd/
 oMMMMMMMMMMMMMMMMMMMMMMMs -`
:MMMMMMMMMMMMMMMMMMMMMMMMM/
NMMMMMMMMMMMMMMMMMMMMMMMMMd
MMMMMMMmdmMMMMMMMMMMMMMMMMd
MMMMMMy` .mMMMMMMMMMMMmho:`
MMMMMMNo/sMMMMMMMNdy+-.`-/
MMMMMMMMMMMMNdy+:.`.:ohmm:
MMMMMMMmhs+-.`.:+ymNMMMy.
MMMMMM/`.-/ohmNMMMMMMy-
MMMMMMNmNNMMMMMMMMmo.
MMMMMMMMMMMMMMMms:`
MMMMMMMMMMNds/.
dhhyys+/-`
";

static ASCII_ART_ARCOLINUX_SMALL: &str = "
          A
         ooo
        ooooo
       ooooooo
      ooooooooo
     ooooo ooooo
    ooooo   ooooo
   ooooo     ooooo
  ooooo  <oooooooo>
 ooooo      <oooooo>
ooooo          <oooo>
";

static ASCII_ART_ARCOLINUX: &str = "
                    /-
                   ooo:
                  yoooo/
                 yooooooo
                yooooooooo
               yooooooooooo
             .yooooooooooooo
            .oooooooooooooooo
           .oooooooarcoooooooo
          .ooooooooo-oooooooooo
         .ooooooooo-  oooooooooo
        :ooooooooo.    :ooooooooo
       :ooooooooo.      :ooooooooo
      :oooarcooo         .oooarcooo
     :ooooooooy           .ooooooooo
    :ooooooooo   /ooooooooooooooooooo
   :ooooooooo      .-ooooooooooooooooo.
  ooooooooo-             -ooooooooooooo.
 ooooooooo-                 .-oooooooooo.
ooooooooo.                     -ooooooooo
";

static ASCII_ART_ARCH: &str = "
                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/
";

static ASCII_ART_ARCH_SMALL: &str = "
      /\\
     /  \\
    /\\   \\
   /      \\
  /   ,,   \\
 /   |  |  -\\
/_-''    ''-_\\
";

static ASCII_ART_ARCH_OLD: &str = "
             __
         _=(SDGJT=_
       _GTDJHGGFCVS)
      ,GTDJGGDTDFBGX0
     JDJDIJHRORVFSBSVL-=+=,_
    IJFDUFHJNXIXCDXDSV,  \"DEBL
   [LKDSDJTDU=OUSCSBFLD.   '?ZWX,
  ,LMDSDSWH'     `DCBOSI     DRDS],
  SDDFDFH'         !YEWD,   )HDROD
 !KMDOCG            &GSU|\\\\_GFHRGO\\'
 HKLSGP'           __\\TKM0\\GHRBV)'
JSNRVW'       __+MNAEC\\IOI,\\BN'
HELK['    __,=OFFXCBGHC\\FD)
?KGHE \\_-#DASDFLSV='    'EF
'EHTI                    !H
 `0F'                    '!
";

static ASCII_ART_ARCH_BOX: &str = "
              ...:+oh/:::..
         ..-/oshhhhhh`   `::::-.
     .:/ohhhhhhhhhhhh`        `-::::.
 .+shhhhhhhhhhhhhhhhh`             `.::-.
 /`-:+shhhhhhhhhhhhhh`            .-/+shh
 /      .:/ohhhhhhhhh`       .:/ohhhhhhhh
 /           `-:+shhh`  ..:+shhhhhhhhhhhh
 /                 .:ohhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /                  `hhhhhhhhhhhhhhhhhhhh
 /      .+o+        `hhhhhhhhhhhhhhhhhhhh
 /     -hhhhh       `hhhhhhhhhhhhhhhhhhhh
 /     ohhhhho      `hhhhhhhhhhhhhhhhhhhh
 /:::+`hhhhoos`     `hhhhhhhhhhhhhhhhhs+`
    `--/:`   /:     `hhhhhhhhhhhho/-
             -/:.   `hhhhhhs+:-`
                ::::/ho/-`
";

static ASCII_ART_ARCH_LABS: &str = "
                     'c'
                    'kKk,
                   .dKKKx.
                  .oKXKXKd.
                 .l0XXXXKKo.
                 c0KXXXXKX0l.
                :0XKKOxxOKX0l.
               :OXKOc. .c0XX0l.
              :OK0o. ...'dKKX0l.
             :OX0c  ;xOx''dKXX0l.
            :0KKo..o0XXKd'.lKXX0l.
           c0XKd..oKXXXXKd..oKKX0l.
         .c0XKk;.l0K0OO0XKd..oKXXKo.
        .l0XXXk:,dKx,.'l0XKo..kXXXKo.
       .o0XXXX0d,:x;   .oKKx'.dXKXXKd.
      .oKXXXXKK0c.;.    :00c'cOXXXXXKd.
     .dKXXXXXXXXk,.     cKx''xKXXXXXXKx'
    'xKXXXXK0kdl:.     .ok; .cdk0KKXXXKx'
   'xKK0koc,..         'c,     ..,cok0KKk,
  ,xko:'.             ..            .':okx;
 .,'.                                   .',.
";

static ASCII_ART_ARCH_MERGE: &str = "
                    y:
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
/+.                                  -o.
";

static ASCII_ART_ARCH_STRIKE: &str = "
                   *   
                  **.
                 ****
                ******
                *******
              ** *******
             **** *******
            ****_____***/*
           ***/*******//***
          **/********///*/**
         **/*******////***/**
        **/****//////.,****/**
       ***/*****/////////**/***
      ****/****    /////***/****
     ******/***  ////   **/******
    ********/* ///      */********
  ,******     // ______ /    ******,
";

static ASCII_ART_ARTIX_SMALL: &str = "
      /\\
     /  \\
    /`'.,\\
   /     ',
  /      ,`\\
 /   ,.'`.  \\
/.,'`     `'.\\
";

static ASCII_ART_ARTIX: &str = "
                   '
                  'o'
                 'ooo'
                'ooxoo'
               'ooxxxoo'
              'oookkxxoo'
             'oiioxkkxxoo'
            ':;:iiiioxxxoo'
               `'.;::ioxxoo'
          '-.      `':;jiooo'
         'oooio-..     `'i:io'
        'ooooxxxxoio:,.   `'-;'
       'ooooxxxxxkkxoooii:-.  `'
      'ooooxxxxxkkkkxoiiiiiji'
     'ooooxxxxxkxxoiiii:'`     .i'
    'ooooxxxxxoi:::'`       .;ioxo'
   'ooooxooi::'`         .:iiixkxxo'
  'ooooi:'`                `'';ioxxo'
 'i:'`                          '':io'
'`                                   `'
";

static ASCII_ART_ARYA: &str = "
                `oyyy/-yyyyyy+
               -syyyy/-yyyyyy+
              .syyyyy/-yyyyyy+
              :yyyyyy/-yyyyyy+
           `/ :yyyyyy/-yyyyyy+
          .+s :yyyyyy/-yyyyyy+
         .oys :yyyyyy/-yyyyyy+
        -oyys :yyyyyy/-yyyyyy+
       :syyys :yyyyyy/-yyyyyy+
      /syyyys :yyyyyy/-yyyyyy+
     +yyyyyys :yyyyyy/-yyyyyy+
   .oyyyyyyo. :yyyyyy/-yyyyyy+ ---------
  .syyyyyy+`  :yyyyyy/-yyyyy+-+syyyyyyyy
 -syyyyyy/    :yyyyyy/-yyys:.syyyyyyyyyy
:syyyyyy/     :yyyyyy/-yyo.:syyyyyyyyyyy
";

static ASCII_ART_BEDROCK: &str = "
--------------------------------------
--------------------------------------
--------------------------------------
---\\\\\\\\\\\\\\\\\\\\\\\\-----------------------
----\\\\\\      \\\\\\----------------------
-----\\\\\\      \\\\\\---------------------
------\\\\\\      \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\------
-------\\\\\\                    \\\\\\-----
--------\\\\\\                    \\\\\\----
---------\\\\\\        ______      \\\\\\---
----------\\\\\\                   ///---
-----------\\\\\\                 ///----
------------\\\\\\               ///-----
-------------\\\\\\////////////////------
--------------------------------------
--------------------------------------
--------------------------------------
";

static ASCII_ART_BITRIG: &str = "
   `hMMMMN+
   -MMo-dMd`
   oMN- oMN`
   yMd  /NM:
  .mMmyyhMMs
  :NMMMhsmMh
  +MNhNNoyMm-
  hMd.-hMNMN:
  mMmsssmMMMo
 .MMdyyhNMMMd
 oMN.`/dMddMN`
 yMm/hNm+./MM/
.dMMMmo.``.NMo
:NMMMNmmmmmMMh
/MN/-------oNN:
hMd.       .dMh
sm/         /ms
";

static ASCII_ART_HASH: &str = "
      +   ######   +
    ###   ######   ###
  #####   ######   #####
 ######   ######   ######

####### '\"###### '\"########
#######   ######   ########
#######   ######   ########

 ###### '\"###### '\"######
  #####   ######   #####
    ###   ######   ###
      ~   ######   ~
";

static ASCII_ART_X_FERIENCE: &str = "
           ``--:::::::-.`
        .-/+++ooooooooo+++:-`
     `-/+oooooooooooooooooo++:.
    -/+oooooo/+ooooooooo+/ooo++:`
  `/+oo++oo.   .+oooooo+.-: +:-o+-
 `/+o/.  -o.    :oooooo+ ```:.+oo+-
`:+oo-    -/`   :oooooo+ .`-`+oooo/.
.+ooo+.    .`   `://///+-+..oooooo+:`
-+ooo:`                ``.-+oooooo+/`
-+oo/`                       :+oooo/.
.+oo:            ..-/. .      -+oo+/`
`/++-         -:::++::/.      -+oo+-
 ./o:          `:///+-     `./ooo+:`
  .++-         `` /-`   -:/+oooo+:`
   .:+/:``          `-:ooooooo++-
     ./+o+//:...../+oooooooo++:`
       `:/++ooooooooooooo++/-`
          `.-//++++++//:-.`
               ``````
";
