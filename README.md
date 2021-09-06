# Raya

Raya is an acoustic raytracer written in rust.

## Benchmarks

### Auditorium
- __Receiver Radius__: 0.25m
- __Max Order__: 50
- __Ray Count__: 10000
- __Triangles__: 280
##### CRAM
<table>
  <thead>
    <tr>
      <th>Model</th>
      <th>Impulse Response</th>
      <th>Raytracing</th>
      <th>IR Calculation</th>
      <th>Total</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="/bench/auditorium/cram/auditorium.json">auditorium.json</a></td>
      <td><a href="/bench/auditorium/cram/auditorium.wav">auditorium.wav</a></td>
      <td>115.42s</td>
      <td>0.35s</td>
      <td>115.77s</td>
    </tr>
  </tbody>
</table>

##### Raya
<table>
  <thead>
    <tr>
      <th>Model</th>
      <th>Impulse Response</th>
      <th>Raytracing</th>
      <th>IR Calculation</th>
      <th>Total</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="/bench/auditorium/raya/auditorium.gltf">auditorium.gltf</a></td>
      <td><a href="/bench/auditorium/raya/auditorium.wav">auditorium.wav</a></td>
      <td>12.56s</td>
      <td>0.17s</td>
      <td>12.73s</td>
    </tr>
  </tbody>
</table>
