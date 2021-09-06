# Raya

Raya is an acoustic raytracer written in rust.

## Usage

```txt
USAGE:
    raya --model <FILE> --output <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --model <FILE>     The 3d model file used (.gltf)
    -o, --output <FILE>    The file path for the calculated impulse response (.wav)
```

### Examples

```sh
raya -m bench/auditorium/raya/auditorium.gltf -o bench/auditorium/raya/auditorium.wav
```

## Benchmarks

### Auditorium
- __Receiver Radius__: 0.25m
- __Max Order__: 50
- __Ray Count__: 10000
- __Triangles__: 56
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

### Shoebox
- __Receiver Radius__: 0.2m
- __Max Order__: 50
- __Ray Count__: 10000
- __Triangles__: 12
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
      <td><a href="/bench/shoebox/cram/shoebox.json">shoebox.json</a></td>
      <td><a href="/bench/shoebox/cram/shoebox.wav">shoebox.wav</a></td>
      <td>22.36s</td>
      <td>0.33s</td>
      <td>22.69s</td>
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
      <td><a href="/bench/shoebox/raya/shoebox.gltf">shoebox.gltf</a></td>
      <td><a href="/bench/shoebox/raya/shoebox.wav">shoebox.wav</a></td>
      <td>4.33s</td>
      <td>0.09s</td>
      <td>4.42s</td>
    </tr>
  </tbody>
</table>
