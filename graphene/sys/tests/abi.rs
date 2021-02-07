// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use graphene_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["graphene-gobject-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(";");
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(";");
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GRAPHENE_EULER_ORDER_DEFAULT", "-1"),
    ("(gint) GRAPHENE_EULER_ORDER_RXYX", "19"),
    ("(gint) GRAPHENE_EULER_ORDER_RXYZ", "28"),
    ("(gint) GRAPHENE_EULER_ORDER_RXZX", "21"),
    ("(gint) GRAPHENE_EULER_ORDER_RXZY", "22"),
    ("(gint) GRAPHENE_EULER_ORDER_RYXY", "25"),
    ("(gint) GRAPHENE_EULER_ORDER_RYXZ", "26"),
    ("(gint) GRAPHENE_EULER_ORDER_RYZX", "20"),
    ("(gint) GRAPHENE_EULER_ORDER_RYZY", "23"),
    ("(gint) GRAPHENE_EULER_ORDER_RZXY", "24"),
    ("(gint) GRAPHENE_EULER_ORDER_RZXZ", "27"),
    ("(gint) GRAPHENE_EULER_ORDER_RZYX", "18"),
    ("(gint) GRAPHENE_EULER_ORDER_RZYZ", "29"),
    ("(gint) GRAPHENE_EULER_ORDER_SXYX", "7"),
    ("(gint) GRAPHENE_EULER_ORDER_SXYZ", "6"),
    ("(gint) GRAPHENE_EULER_ORDER_SXZX", "9"),
    ("(gint) GRAPHENE_EULER_ORDER_SXZY", "8"),
    ("(gint) GRAPHENE_EULER_ORDER_SYXY", "13"),
    ("(gint) GRAPHENE_EULER_ORDER_SYXZ", "12"),
    ("(gint) GRAPHENE_EULER_ORDER_SYZX", "10"),
    ("(gint) GRAPHENE_EULER_ORDER_SYZY", "11"),
    ("(gint) GRAPHENE_EULER_ORDER_SZXY", "14"),
    ("(gint) GRAPHENE_EULER_ORDER_SZXZ", "15"),
    ("(gint) GRAPHENE_EULER_ORDER_SZYX", "16"),
    ("(gint) GRAPHENE_EULER_ORDER_SZYZ", "17"),
    ("(gint) GRAPHENE_EULER_ORDER_XYZ", "0"),
    ("(gint) GRAPHENE_EULER_ORDER_XZY", "3"),
    ("(gint) GRAPHENE_EULER_ORDER_YXZ", "4"),
    ("(gint) GRAPHENE_EULER_ORDER_YZX", "1"),
    ("(gint) GRAPHENE_EULER_ORDER_ZXY", "2"),
    ("(gint) GRAPHENE_EULER_ORDER_ZYX", "5"),
    ("GRAPHENE_HAS_GCC", "1"),
    ("GRAPHENE_HAS_SCALAR", "1"),
    ("GRAPHENE_HAS_SSE", "1"),
    ("GRAPHENE_PI", "3.141593"),
    ("GRAPHENE_PI_2", "1.570796"),
    ("(gint) GRAPHENE_RAY_INTERSECTION_KIND_ENTER", "1"),
    ("(gint) GRAPHENE_RAY_INTERSECTION_KIND_LEAVE", "2"),
    ("(gint) GRAPHENE_RAY_INTERSECTION_KIND_NONE", "0"),
    ("GRAPHENE_SIMD_S", "sse"),
    ("GRAPHENE_VEC2_LEN", "2"),
    ("GRAPHENE_VEC3_LEN", "3"),
    ("GRAPHENE_VEC4_LEN", "4"),
];