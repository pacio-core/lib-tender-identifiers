# pledges-sig-lib

Curve25519 signatures library for Rust / Typescript / Android / iOS.

### Prerequisites

- On mac/linux, make sure you have the packages `build-essentials`, `pkg-config` installed on your system (for Rust).
- In order to build for iOS, you need a mac.

Other dependencies will be installed automatically (when you run the makefile commands)

## Usage

### Rust

<details>
  <summary>Installation</summary>
  <p>
  In your project's `Cargo.toml`, add the following line under `[dependencies]`:

```toml
ed25519xp = { git="ssh://git@github.com:pacio-core/pledges-sig-lib.git" }
```

  </p>
</details>

<details>
  <summary>API</summary>
  <p> 
    See docs.rs
  </p>
</details>

### Typescript

<details>
  <summary>Installation</summary>
  <ol>
    <li>Copy `android/ed25519lib/` just under the root of your android app project</li>
    <li>
      Add this to your App's main build.gradle file:
      <blockquote>
      dependencies {
          implementation project(':ed25519lib')
      } 
      </blockquote>
    </li>
  </ol>
</details>

<details>
  <summary>API</summary>
  <ul> 
    <li>generateKeyPair(seed) -> KeyPair</li>
    <li>sign(privateKey, message, [random]) -> Signature</li>
    <li>verify(publicKey, message, signature) -> true | false</li>
    <li>serializeKeyPair(keyPair) -> String</li>
    <li>deserializeKeyPair(String) -> KeyPair</li>
  </ul>
</details>

### Android

<details>
  <summary>Installation</summary>
  <ol>
    <li>Copy `android/ed25519lib/` just under the root of your android app project</li>
    <li>
      Add this to your App's main build.gradle file:
      <pre><code>
      dependencies {
          implementation project(':ed25519lib')
      } 
      </code></pre>
    </li>
    <br/>
    <details>
      <summary>Example basic usage</summary>
      In an Activity, import both the functions you need and loadLibEd25519(), and do:
      <pre><code>
        import com.pacio.ed25519lib.keypair_from_phrase
        import com.pacio.ed25519lib.loadLibEd25519
        class MainActivity : AppCompatActivity() {
            override fun onCreate(savedInstanceState: Bundle?) {
                super.onCreate(savedInstanceState)
                setContentView(R.layout.activity_main)
                setSupportActionBar(toolbar)
                loadLibEd25519()
                findViewById<TextView>(R.id.txt).let {
                    it?.text = keypair_from_phrase("Hello Luka")
                }
            }
            override fun onCreateOptionsMenu(menu: Menu): Boolean {
                // Inflate the menu; this adds items to the action bar if it is present.
                menuInflater.inflate(R.menu.menu_main, menu)
                return true
            }
            override fun onOptionsItemSelected(item: MenuItem): Boolean {
                // Handle action bar item clicks here. The action bar will
                // automatically handle clicks on the Home/Up button, so long
                // as you specify a parent activity in AndroidManifest.xml.
                return when (item.itemId) {
                    R.id.action_settings -> true
                    else -> super.onOptionsItemSelected(item)
                }
            }
        }
      </code></pre>
    </details>

  </ol>
</details>

<details>
  <summary>API</summary>
  <ul>
    <li>keypair_from_phrase(phrase_utf8: JString) -> (keyPair: JByteArray)</li>
    <li>pubKey_from_pair_bytes(keypair: JByteArray) -> (pubKey: JByteArray)</li>
    <li>sign(message: JByteArray, keypair: JByteArray) -> (signature: JByteArray)</li>
    <li>verify(message: JByteArray, pubKey: JByteArray, sig: JByteArray) -> (isValid: boolean)</li>
    <li>seed_from_phrase(phrase_utf8: JString) -> (seed_bytes: JByteArray)</li>
  </ul>
</details>

### iOS

<details>
  <summary>Installation</summary>
  <ul>
    <li>copy `ios/libs/` and `ios/include/` into the top of you folder</li>
    <li>
    In Xcode, in your project settings -> General -> Frameworks, libraries, and embedded content, <br/>
        add the file `ios/libs/libed25519xp.a` (if it doesn't appear, add it a second time)
    </li>
    <li>
        In Xcode, in your project settings -> Build Settings, <br/>
        <ul>
            <li>set `Header Search Paths` to `../include`</li>
            <li>set `Library Search Paths` to `../libs`</li>
            <li>set `Objective-C Bridging Header` to `../include`</li>
        </ul>
    </li>

  </ul>
</details>

<details>
  <summary>API</summary>
  <ul>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> (keyPair: RustByteSlice)</li>
    <li>pubKey_from_pair_bytes(keypair: RustByteSlice) -> (pubKey: RustByteSlice)</li>
    <li>sign(message: RustByteSlice, keypair: RustByteSlice) -> (signature: RustByteSlice)</li>
    <li>verify(message: RustByteSlice, pubKey: RustByteSlice, sig: RustByteSlice) -> (isValid: bool)</li>
  </ul>
</details>

### Building from source

#### for typescript, run

```shell
make ts.build
```

#### for android, run

```shell
make a.build
```

#### for ios, run

```shell
make ios.build
```
