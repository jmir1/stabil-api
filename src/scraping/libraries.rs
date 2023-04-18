use super::models::Library;

pub static LIBRARIES: phf::Map<&'static str, (i32, &'static str)> = phf::phf_map! {
    "Staats- und Universitätsbibliothek" => (2, "(standort_iln_str_mv:22\\\\:SUB+OR+standort_iln_str_mv:22\\\\:SUB-*)"),
    "FB Sprache, Literatur, Medien" => (200, "(standort_iln_str_mv:22\\\\:18\\\\/156+OR+standort_iln_str_mv:22\\\\:18\\\\/61+OR+standort_iln_str_mv:22\\\\:18\\\\/59+OR+standort_iln_str_mv:22\\\\:18\\\\/12+OR+standort_iln_str_mv:22\\\\:18\\\\/21+OR+standort_iln_str_mv:22\\\\:18\\\\/39+OR+standort_iln_str_mv:22\\\\:18\\\\/294+OR+standort_iln_str_mv:22\\\\:18\\\\/219+OR+standort_iln_str_mv:22\\\\:18\\\\/156-*)"),
    "ZB Recht" => (1089712717, "(standort_iln_str_mv:22\\\\:18\\\\/304)"),
    "ZB Philosophie, Geschichte u. Klass. Philologie" => (155, "(standort_iln_str_mv:22\\\\:18\\\\/309-*)"),
    "FB Kulturwissenschaften" => (1168358212, "(standort_iln_str_mv:22\\\\:18\\\\/308+OR+standort_iln_str_mv:22\\\\:18\\\\/308-*)"),
    "Asien-Afrika-Inst." => (1067963281, "(standort_iln_str_mv:22\\\\:18\\\\/303+OR+standort_iln_str_mv:22\\\\:18\\\\/311+OR+standort_iln_str_mv:22\\\\:18\\\\/303-E)"),
    "Ärztliche Zentralbibliothek" => (347, "(standort_iln_str_mv:22\\\\:18\\\\/64+OR+standort_iln_str_mv:22\\\\:18\\\\/64-*+OR+standort_iln_str_mv:22\\\\:18\\\\/297)"),
    "Martha-Muchow-Bibliothek" => (1163089918, "(standort_iln_str_mv:22\\\\:18\\\\/307-*+OR+standort_iln_str_mv:22\\\\:18\\\\/310+OR+standort_iln_str_mv:22\\\\:18\\\\/307)"),
    "FB Geowissenschaften" => (112, "(standort_iln_str_mv:22\\\\:18\\\\/57+OR+standort_iln_str_mv:22\\\\:18\\\\/306+OR+standort_iln_str_mv:22\\\\:18\\\\/24+OR+standort_iln_str_mv:22\\\\:H8+OR+standort_iln_str_mv:22\\\\:18\\\\/314+OR+standort_iln_str_mv:22\\\\:18\\\\/306-*+OR+standort_iln_str_mv:22\\\\:18\\\\/57-*)"),
    "FB Wirtschaftswissenschaften" => (78, "(standort_iln_str_mv:22\\\\:18\\\\/261+OR+standort_iln_str_mv:22\\\\:18\\\\/261-*)"),
    "Nordostinst." => (1283848727, "(standort_iln_str_mv:22\\\\:18\\\\/313+OR+standort_iln_str_mv:22\\\\:18\\\\/313-*)"),
    "FB Theologie" => (71, "(standort_iln_str_mv:22\\\\:18\\\\/161+OR+standort_iln_str_mv:22\\\\:18\\\\/161-*)"),
    "FB Sozialwissenschaften" => (315, "(standort_iln_str_mv:22\\\\:18\\\\/76+OR+standort_iln_str_mv:22\\\\:18\\\\/76-*)"),
    "FB Biologie" => (369, "(standort_iln_str_mv:22\\\\:18\\\\/305+OR+standort_iln_str_mv:22\\\\:18\\\\/19)"),
    "Forschungsstelle für Zeitgeschichte HH" => (105, "(standort_iln_str_mv:22\\\\:H250)"),
    "FB Informatik" => (72, "(standort_iln_str_mv:22\\\\:18\\\\/228+OR+standort_iln_str_mv:22\\\\:18\\\\/228-*)"),
    "FB Mathematik" => (236, "(standort_iln_str_mv:22\\\\:18\\\\/263)"),
    "Linga-Bibliothek" => (0, "(standort_iln_str_mv:22\\\\:SUB-LINGA)"),
    "Musikwissenschaftliches Inst." => (252, "(standort_iln_str_mv:22\\\\:18\\\\/114)"),
    "Inst. für die Geschichte der deutschen Juden" => (179, "(standort_iln_str_mv:\"22\\\\:H 227\")"),
    "FB Physik" => (267, "(standort_iln_str_mv:22\\\\:18\\\\/47-*+OR+standort_iln_str_mv:22\\\\:18\\\\/269+OR+standort_iln_str_mv:22\\\\:18\\\\/270+OR+standort_iln_str_mv:22\\\\:18\\\\/47)"),
    "FB Chemie" => (48, "(standort_iln_str_mv:22\\\\:18\\\\/48+OR+standort_iln_str_mv:22\\\\:18\\\\/48-*)"),
    "Inst. für Friedensforschung und Sicherheitspolitik" => (182, "(standort_iln_str_mv:22\\\\:18\\\\/226)"),
    "Hamburger Sternwarte" => (138, "(standort_iln_str_mv:22\\\\:18\\\\/15)"),
    "HH Bibl. für Universitätsgeschichte" => (124, "(standort_iln_str_mv:22\\\\:18\\\\/296)"),
    "Bibl. für deutschsprachige Exilliteratur" => (120, "(standort_iln_str_mv:22\\\\:18\\\\/290+OR+standort_iln_str_mv:22\\\\:SUB-OS)"),
    "ZB Frauen und Gender Studies" => (217, "(standort_iln_str_mv:22\\\\:18\\\\/261-k)"),
    "Arbeitsstelle für Hamburgische Geschichte" => (185, "(standort_iln_str_mv:22\\\\:18\\\\/309-s*)"),
    "Bibl. deutsche Gebärdensprache" => (365, "(standort_iln_str_mv:22\\\\:18\\\\/295)"),
    "Centre for the Study of Manuscript Cultures" => (1283848716, "(standort_iln_str_mv:22\\\\:18\\\\/303-SFB)"),
    "Inst. für Jüdische Philosophie und Religion" => (1283848732, "(standort_iln_str_mv:22\\\\:18\\\\/309-j+OR+standort_iln_str_mv:22\\\\:18\\\\/309-m)"),
    "Europa-Kolleg" => (0, "(standort_iln_str_mv:22\\\\:18\\\\/254)"),
    "Hans-Bredow-Institut" => (150, "(standort_iln_str_mv:22\\\\:18\\\\/174)"),
    "Missionsakademie" => (0, "(standort_iln_str_mv:22\\\\:18\\\\/239)"),
    "Gerd-Bucerius-Bibliothek" => (0, "(standort_iln_str_mv:22\\\\:H22)"),
};

pub fn to_library(str: &str) -> Library {
    let (id, filter_b) = LIBRARIES.get(str).unwrap_or(&(0, "")).to_owned();
    let filter = filter_b.to_owned();
    let name = str.to_string();
    Library { id, name, filter }
}
