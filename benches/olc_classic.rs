#[macro_use]
extern crate criterion;

extern crate aeroscore;
extern crate igc;

use aeroscore::olc;
use criterion::Criterion;

struct Point {
    latitude: f32,
    longitude: f32,
    altitude: i16,
}

impl aeroscore::Point for Point {
    fn latitude(&self) -> f32 {
        self.latitude
    }
    fn longitude(&self) -> f32 {
        self.longitude
    }
    fn altitude(&self) -> i16 {
        self.altitude
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("olc_classic", |b| {
        b.iter(|| {
            let fixes = include_str!("../tests/fixtures/2017-08-14-fla-6ng-01.igc")
                .lines()
                .filter(|l| l.starts_with('B'))
                .filter_map(|line| {
                    igc::records::BRecord::parse(&line)
                        .ok()
                        .map(|record| Point {
                            latitude: record.pos.lat.into(),
                            longitude: record.pos.lon.into(),
                            altitude: record.pressure_alt,
                        })
                })
                .collect::<Vec<_>>();

            olc::optimize(&fixes).unwrap()
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10);

    targets = criterion_benchmark
}
criterion_main!(benches);
