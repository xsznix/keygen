# keygen

An(other) algorithm for generating optimal keyboard layouts.

This code follows the simulated annealing method used in [Carpalx](http://mkweb.bcgsc.ca/carpalx/?simulated_annealing), but with a different model. Here, we try to maximise comfort by minimising the actions that cause discomfort---stretching or compressing the hand in uncomfortable ways. In addition to the base cost of the key due to its position, we also include:

* A penalty for using the same finger twice on different keys. Example (QWERTY): ED/DE, LO. Using the same finger twice is the second slowest thing you can do on a keyboard. An extra penalty is awarded if one of the keys in the combination is in a centre column, since lateral movements are slower.
* A penalty for jumping from the top row to the bottom row or from the bottom row to the top row on the same finger. Example: CE, UN. Jumping across the home row is the slowest thing you can do on a keyboard.
* A penalty for jumping from top to bottom row or from bottom to top row on consecutive fingers, excluding middle--index. Example: EX. This isn't as bad as jumping the home row on the same finger, but causes your fingers to awkwardly stretch to reach the second key.
* A penalty for jumping from top to bottom row or from bottom to top row on the same hand. Such a jump, even if not on the same or consecutive fingers, causes your hands to bend awkwardly.
* A penalty for an awkward pinky/ring combination where the pinky reaches above the ring finger. Example: QA/AQ, PL/LP, ZX/XZ. Since the pinky is longer than the ring finger, this causes your hand to awkwardly compress.
* A penalty for reversing a roll at the end of the hand, i.e. using the ring, pinky, then middle finger of the same hand. Examples: WAD. Since the movement of the ring finger is partially dependent on that of the middle finger, this motion is particularly tricky and therefore inaccurate. In Dvorak, typing "install" may sometimes result in "instnall" or "insntall" as a result of this dependency.
* A penalty for using the same hand four times in a row. Examples: EVER, WERE, LOOK. Using the same hand for too many letters in a row fatigues the hand creates the opportunity for error.
* A penalty for alternating hands three times in a row. Examples: WITH, IGHT, WHEN. Alternating too often may cause the timing of the alternation to fall apart, for example resulting in "teh" or "hte" for "the".
* A slight penalty for rolling outwards, accompanied by a slight award (negative penalty) for rolling inwards, since a rolling in motion feels more natural than a rolling out motion.

## Installing and running

You'll need a recent-ish version of [Rust](https://www.rust-lang.org/).

Then: `cargo run -- run corpus/books.short.txt`.

## Installing the (upcoming) optimal keyboard layout

If you're crazy enough to want to try this, you're probably smart enough to figure out how to install custom keyboards on your system of choice.

## Credits

The simulated annealing algorithm and corpus are taken from Carpalx by Martin Krzywinski.

## Other alternate keyboard layouts

mdickens has a good list of them [here](http://mdickens.me/typing/alternative_layouts.html).

## Licence

MIT