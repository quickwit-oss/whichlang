use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

// A random ascii string of length 100 chars.
const ASCII_SHORT: &str = "It is a long established fact";
const ASCII_MEDIUM: &str = "It is a long established fact that a reader will be distracted by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English. Many desktop publishing packages and web page editors now use Lorem Ipsum as their default model text, and a search for 'lorem ipsum' will uncover many web sites still in their infancy. Various versions have evolved over the years, sometimes by accident, sometimes on purpose (injected humour and the like).";
const JP_SHORT: &str = "日本ごです。　とても素敵な言葉ですね";
const JP_MEDIUM: &str = "日本ごです。　和名の由来は、太陽の動きにつれてその方向を追うように花が回るといわれたことから。ただしこの動きは生長に伴うものであるため、実際に太陽を追って動くのは生長が盛んな若い時期だけである。若いヒマワリの茎の上部の葉は太陽に正対になるように動き、朝には東を向いていたのが夕方には西を向く。日没後はまもなく起きあがり、夜明け前にはふたたび東に向く。この運動はつぼみを付ける頃まで続くが、つぼみが大きくなり花が開く素敵な言葉ですね.";

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("whichlang");
    group
        .throughput(Throughput::Bytes(ASCII_SHORT.len() as u64))
        .bench_with_input("inference_short", ASCII_SHORT, |b, text| {
            b.iter(|| whichlang::detect_language(black_box(text)));
        });
    group
        .throughput(Throughput::Bytes(ASCII_MEDIUM.len() as u64))
        .bench_with_input("inference_long", ASCII_MEDIUM, |b, text| {
            b.iter(|| whichlang::detect_language(black_box(text)));
        });
    group
        .throughput(Throughput::Bytes(JP_SHORT.len() as u64))
        .bench_with_input("inference_jp_short", JP_SHORT, |b, text| {
            b.iter(|| whichlang::detect_language(black_box(text)));
        });
    group
        .throughput(Throughput::Bytes(JP_MEDIUM.len() as u64))
        .bench_with_input("inference_jp_medium", JP_MEDIUM, |b, text| {
            b.iter(|| whichlang::detect_language(black_box(text)));
        });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
