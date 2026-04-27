fn main() {
    // Ce code ne s'exécute que si on compile pour Windows
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        // Assurez-vous que le chemin vers le fichier .ico est correct
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}