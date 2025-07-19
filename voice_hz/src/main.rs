use std::f32::consts::PI;
use std::time::Duration;
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<()> {
    // 1. 拿到預設 Host 跟預設輸出裝置
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("找不到預設輸出設備，請確認聲音裝置");

    // 2. 讀取裝置支援的預設設定（SampleRate、Channels）
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // 3. 你想播幾 Hz？可以改成讀參數或互動輸入
    let freq_hz = 10.0; // A4 音高，測試用

    // 4. 產生正弦波產生器：sample_clock 控制相位
    let mut sample_clock = 0f32;
    let mut next_sample = move || {
        // 0..sample_rate-1 走一圈就完成一個週期
        let value = (sample_clock * 2.0 * PI * freq_hz / sample_rate).sin();
        sample_clock = (sample_clock + 1.0) % sample_rate;
        value
    };

    // 5. 根據 SampleFormat 建串流（只示範 f32；i16/u16 同理）
    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _| write_data(data, channels, &mut next_sample),
        move |err| eprintln!("流發生錯誤: {}", err),
    )?;

    // 6. 播起來，維持一段時間後結束
    stream.play()?;
    println!("正在播放 {:.1} Hz 的正弦波，3 秒後自動停止…", freq_hz);
    std::thread::sleep(Duration::from_secs(300));
    Ok(())
}

// 把產生的 sample 塞到每個 channel
fn write_data(output: &mut [f32], channels: usize, generator: &mut dyn FnMut() -> f32) {
    for frame in output.chunks_mut(channels) {
        let sample = generator();
        for slot in frame.iter_mut() {
            *slot = sample;
        }
    }
}
