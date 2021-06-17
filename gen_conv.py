from romkan import *

print("use phf::phf_map;\n")
print("use lazy_static::lazy_static;\n")
print("use regex::Regex;\n")

for MAP in ("ROMKAN", "ROMKAN_H", "KANROM", "KANROM_H", "TO_HEPBURN", "TO_KUNREI"):
    script = f"""\
print("pub static {MAP}: phf::Map<&'static str, &'static str> = phf_map! {{")
for r, k in {MAP}.items():
    print(f'    "{{r}}" => "{{k}}",')
print("}};\\n")
"""
    exec(script)

print("lazy_static! {")
statements = ""
for PAT in [p for p in dir() if "PAT" in p]:
    script = f"""\
pattern = {PAT}.pattern
print(f'''\
    pub static ref {PAT}: Regex = Regex::new("{{pattern}}").unwrap();''')"""
    exec(script)
print("}")