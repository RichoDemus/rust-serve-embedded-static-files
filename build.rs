use static_files::resource_dir;

fn main() {
    resource_dir("./web/dist")
        .build()
        .expect("loading resource failed");
}
