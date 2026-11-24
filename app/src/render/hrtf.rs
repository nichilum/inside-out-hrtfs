use sofar::reader::{Filter, OpenOptions};
use sofar::render::{FilterTransition, Renderer, RendererPlan};

fn main() {
    let open = |path| OpenOptions::new().sample_rate(44100.0).open(path).unwrap();
    let sofa_a = open("MIT_KEMAR_normal_pinna.sofa");
    let sofa_b = open("HRTF_raw.sofa");

    let filt_len = sofa_a.filter_len().max(sofa_b.filter_len());
    let mut filter = Filter::new(filt_len);

    let plan = RendererPlan::builder(filt_len)
        .with_sample_rate(sofa_a.sample_rate())
        .with_partition_len(64)
        .build()
        .unwrap();

    let (mut publisher, mut render) = Renderer::builder(&plan)
        .with_filter_transition(FilterTransition::crossfade(32))
        .with_max_delay(sofa_a.max_delay().max(sofa_b.max_delay()))
        .build()
        .unwrap()
        .into_realtime();

    let mut input: Vec<f32> = hound::WavReader::open("input.wav")
        .unwrap()
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / 32768.0)
        .collect();
    input.resize(input.len().div_ceil(256) * 256, 0.0);
    let mid = input.len() / 256 / 2;

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();

    let el = 15f32.to_radians();
    let dir = |i: usize| {
        let t = i as f32 * 256.0 / 44100.0;
        let az = 20f32.to_radians() * (std::f32::consts::TAU * 0.25 * t).sin();
        (el.cos() * az.cos(), el.cos() * az.sin(), el.sin())
    };

    let mut left = vec![0.0; 256];
    let mut right = vec![0.0; 256];

    for (i, block) in input.chunks(256).enumerate() {
        let sofa = if i < mid { &sofa_a } else { &sofa_b };
        let (x, y, z) = dir(i);
        sofa.filter(x, y, z, &mut filter);
        publisher.publish_filter(&filter).unwrap();

        render.process_block(block, &mut left, &mut right).unwrap();
        for (l, r) in left.iter().zip(&right) {
            writer.write_sample(*l).unwrap();
            writer.write_sample(*r).unwrap();
        }
    }
    writer.finalize().unwrap();
}
