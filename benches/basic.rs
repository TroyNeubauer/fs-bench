use std::{
    ffi::CStr,
    io::Write,
    os::unix::prelude::OsStrExt,
    path::PathBuf,
    time::{Duration, Instant},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create + small write", |b| {
        let root_path = "./.bench_tmp";
        let mut file_path = PathBuf::from(root_path);
        file_path.push("A.file");
        std::fs::create_dir_all(root_path).unwrap();
        let _ = std::fs::remove_file(&file_path);
        let data = [0u8; 1024];

        b.iter_custom(|iterations| {
            let mut time = Duration::from_secs_f64(0.0);
            for _ in 0..iterations {
                let start = Instant::now();
                let mut file = std::fs::File::create(&file_path).unwrap();
                file.write_all(&data).unwrap();
                file.sync_all().unwrap();

                time += start.elapsed();
                drop(black_box(file));

                let _ = std::fs::remove_file(&file_path);
            }
            time
        })
    });

    c.bench_function("delete", |b| {
        let root_path = "./.bench_tmp";
        let mut file_path = PathBuf::from(root_path);
        file_path.push("B.file");

        std::fs::create_dir_all(root_path).unwrap();
        let _ = std::fs::remove_file(&file_path);
        let data = [0u8; 1024];

        b.iter_custom(|iterations| {
            let mut time = Duration::from_secs_f64(0.0);
            for _ in 0..iterations {
                let mut file = std::fs::File::create(&file_path).unwrap();
                file.write_all(&data).unwrap();
                file.sync_all().unwrap();
                drop(black_box(file));

                let start = Instant::now();

                let _ = std::fs::remove_file(&file_path);
                // Sadly we have to sync the entire fs, since there is no way to hang onto a fd
                // after deletion which we could use to fsync just the deleted file
                nix::unistd::sync();

                time += start.elapsed();
            }
            time
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
