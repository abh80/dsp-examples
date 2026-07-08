Limiter
====

Types of limiter
===
Hard clipper — y = clamp(x, −T, T). Instantaneous, zero lookahead, cheapest. Odd harmonics + aliasing. Still used deliberately on transients (kick, snare) for loudness.
Soft clipper / waveshaper — static nonlinearity (tanh, cubic, arctan — the kind of curve you already visualized). No time state; smoother harmonic rolloff, still aliases unless oversampled.
Feedforward peak limiter w/ ballistics — the loop above. Gain from input.
Feedback limiter — gain derived from the output. Classic analog character, self-correcting, but can't guarantee a hard brickwall (the peak has already passed before it's measured).
Lookahead limiter — feedforward + delay. First design that's a true brickwall without instantaneous distortion.
True-peak (ISP) limiter — oversampled lookahead. The delivery standard.
Multiband limiter — split into bands (crossover or FFT), limit each independently, recombine. Stops a bass transient from ducking the whole mix, and confines distortion to the offending band.