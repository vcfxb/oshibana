fn main() {
    built::write_built_file().expect("built tool");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/favicon/icons/icon.ico");
        res.compile().expect("winres");
    }
}
