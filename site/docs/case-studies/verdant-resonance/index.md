# Powering Verdant Resonance in the browser

      

See how Verdant Engine runs a 60fps 3D sanctuary with clutter-free controls.

      Safety factors of the seven sanctuary blueprints at t_obs. Every constructed monument and kinetic rover exceeds the 2.0 minimum structural safety factor.The data in this figure
|  | x |
| --- | --- |
| Harmonic Rotunda | 3 |
| Meridian Sundial | 3 |
| --- | --- |
| Modal Chime Pergola | 4 |
| Quartz Torsion Chamber | 4 |
| --- | --- |
| Lagoon Pier Causeway | 3 |
| Travertine Paths | 4 |
| --- | --- |
| Sentinel Rover | 3 |
      
      

## Keeping the 3D viewport front and center

      

Early iterations of Verdant Resonance suffered from cluttered menu controls that distracted players from exploring the 3D sanctuary.

      

Extracting Verdant Engine moved simulation, culling, and WebGPU or WebGL2 rendering into an OffscreenCanvas worker backed by verdant-wasm.

      

The studio leaves clean actions beside the 3D stage: epoch scrubbing, run mode, camera mode, spatial audio, GLB export, and #vdnt= snapshot sharing.

      

## Exploring the sanctuary across devices

      

In Orbit mode, drag or scroll to inspect the rotating Cardan astrolabe rings inside the Harmonic Rotunda or follow the Perimeter Sentinel Rover.

      

Switch to Walk mode on desktop to traverse the amphitheater terrain at eye level with WASD or arrow keys.

      

Enable Spatial Audio to hear Euler-Bernoulli chime harmonics attenuate smoothly with your 3D distance from the pergola.

      

## Verified telemetry in a Web Worker

      

Every epoch change and blueprint selection queries the signed WebAssembly verifier inside a background worker without blocking 60fps rendering.

    
      
        Previous: World expansion and architectural statics
