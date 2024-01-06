# Why let and var and const?

(Answer: just use let)

```rust
// Statics and consts are SCREAMING_SNAKE_CASE:
static EVIL_EYE: Mutex<EvilEye> = Mutex::new(EvilEye::new());
const NUMBER_OF_EYES: usize = 2;

// Types are CapitalCamelCase:
struct HelloStructureMyOldFriend;
enum IveComeToEnumYouAgain;
type EvenThoughYouAreATypeAlias = i32;
enum EnumVariantsAre {
    Also,
    Types,
}

// Everything else is snake_case
fn everything_else();
```


```js
var x = 1;



let y = 2;



const z = 3;
// can't do this:
z = 4;






var x = 5;
console.log(x); // 5
function bleh() {
    var x = 10;
    console.log(x); // 10
    if(true) {
        var x = 25;
        console.log(x); // 25
    }
    console.log(x); // 25
}
console.log(x); // 5


let x = 5;
console.log(x); // 5
function bleh() {
    let x = 10;
    console.log(x); // 10
    if(true) {
        let x = 25;
        console.log(x); // 25
    }
    console.log(x); // 10
}
console.log(x); // 5



```

# How big is an int?

PDP-7: 18 bit words, 9 bit characters

```c
int big_word;
char little_word;
```
