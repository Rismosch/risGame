Write-Host "checking preconditions..."

$sdl2_dll = "$PSScriptRoot/../SDL2.dll"

$sdl2_dll_exists = Test-Path $sdl2_dll

if (!$sdl2_dll_exists) {
    throw "could not find ``SDL2.dll`` in the root directory"
}

Write-Host "generating build info..."

$build_info_path = "$PSScriptRoot/../crates/ris_data/src/info/build_info.rs"

function RunCommand {
    param (
        $command
    )

    try {
        return Invoke-Expression $command
    }
    catch {
        return "error while running ``$command``"
    }
}

$git_repo = RunCommand "git config --get remote.origin.url"
$git_commit = RunCommand "git rev-parse HEAD"
$git_branch = RunCommand "git rev-parse --abbrev-ref HEAD"

$rustc_version = RunCommand "rustc --version"
$rustup_toolchain = RunCommand "rustup show active-toolchain"

$build_date = Get-Date -Format "o"

$build_info_content =
"// DO NOT COMMIT CHANGES TO THIS FILE.
// DO NOT MODIFY THIS FILE.
//
// THE CONTENTS OF THIS FILE ARE AUTOMATICALLY GENERATED BY THE BUILD SCRIPT.
//
// I highly recommend you run the following git command:
// git update-index --assume-unchanged crates/ris_data/src/info/build_info.rs
//
// Doc: https://git-scm.com/docs/git-update-index#_using_assume_unchanged_bit

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct BuildInfo {
    git_repo: String,
    git_commit: String,
    git_branch: String,
    rustc_version: String,
    rustup_toolchain: String,
    build_profile: String,
    build_date: String,
}

pub fn build_info() -> BuildInfo {
    BuildInfo {
        git_repo: String::from(`"$git_repo`"),
        git_commit: String::from(`"$git_commit`"),
        git_branch: String::from(`"$git_branch`"),
        rustc_version: String::from(`"$rustc_version`"),
        rustup_toolchain: String::from(`"$rustup_toolchain`"),
        build_profile: profile(),
        build_date: String::from(`"$build_date`"),
    }
}

#[cfg(debug_assertions)]
fn profile() -> String {
    String::from(`"debug`")
}

#[cfg(not(debug_assertions))]
fn profile() -> String {
    String::from(`"release`")
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, `"Build`")?;
        writeln!(f, `"git repo:     {}`", self.git_repo)?;
        writeln!(f, `"git commit:   {}`", self.git_commit)?;
        writeln!(f, `"git branch:   {}`", self.git_branch)?;
        writeln!(f, `"compiler:     {}`", self.rustc_version)?;
        writeln!(f, `"toolchain:    {}`", self.rustup_toolchain)?;
        writeln!(f, `"profile:      {}`", self.build_profile)?;
        writeln!(f, `"build date:   {}`", self.build_date)?;

        Ok(())
    }
}"

Set-Content -Path $build_info_path -Value $build_info_content

Write-Host "compiling workspace..."

cargo clean
cargo build -r

Write-Host "moving files..."

$target_directory = "$PSScriptRoot/../target/release"
$final_directory = "$PSScriptRoot/build"

if (Test-Path $final_directory) {
    Remove-Item -Recurse -Force $final_directory
}

New-Item -Path $final_directory -ItemType Directory | out-null

Copy-Item "$target_directory/app.exe" -Destination "$final_directory/app.exe"
Copy-Item "$target_directory/ris_engine.exe" -Destination "$final_directory/ris_engine.exe"
Copy-Item $sdl2_dll -Destination "$final_directory/SDL2.dll"

$resolved_final_directory = Resolve-Path $final_directory

Write-Host "Done! Final build can be found under ``$resolved_final_directory``"