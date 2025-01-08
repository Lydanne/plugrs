use plugin_interface::Plugin;
use plugin_macros::export_plugin;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use walkdir::WalkDir;

#[export_plugin]
pub struct FileAnalyzer {
    stats: Arc<Mutex<HashMap<String, usize>>>,
}

impl FileAnalyzer {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn analyze_directory(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(ext) = entry.path().extension() {
                    let ext = ext.to_string_lossy().to_string();
                    let mut stats = self.stats.lock().unwrap();
                    *stats.entry(ext).or_insert(0) += 1;
                }
            }
        }
        Ok(())
    }
}

impl Plugin for FileAnalyzer {
    fn name(&self) -> String {
        "File Analyzer Plugin".to_string()
    }

    fn execute(&self) -> i32 {
        self.analyze_directory(".").unwrap();
        println!("文件分析结果:");
        println!("----------------------------------------");
        let stats = self.stats.lock().unwrap();
        for (ext, count) in stats.iter() {
            println!("{:10} : {} 个文件", ext, count);
        }
        println!("----------------------------------------");
        stats.len() as i32
    }
}
