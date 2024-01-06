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

Rust:
```
i8 // is 8 bits
i16 // is 16 bits
i32 // is 32 bits
i64 // is 64 bits
i128 // is 128 bits
isize // is big enough to hold any size OR pointer (and is a special type, so the compiler can see when you did it wrong)
```


PDP-7: 18 bit words, 9 bit characters
```c
int big_word; // 18 bits
short int not_as_little_word; // 9 bits
char little_word; // 9 bits
```

PDP-11: 16 bit words, 8 bit characters
```c
int big_word; // 16 bits
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
```

VAX-11: 32 bit words, 8 bit characters
```c
int big_word; // 16 bits
long int bigger_word; // 32 bits
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
long long int biggerer_word; // 64 bits
```

PDP-10: 36 bit words, 18 bit addresses, 6 bit characters "PARIS "! :D
```c
//int big_word; // 36 bits
int big_word; // 18 bit half word
long int bigger_word; // 36 bits
short int not_as_little_word; // 18 bits
char little_word; // 6 bits
```

MIT Lincoln Laboratory TX-2: 36 bit words, 36 bit addresses, 9 bit characters
```c
int big_word; // also 36 bits
long int bigger_word; // 36 bits
short int not_as_little_word; // 18 bits
char little_word; // 9 bits
```

Intel 8088: 16 bit words AND addresses, 8 bit characters, but you can use Magic to do 32 bit math!
```c
int big_word; // 16 bits
long int bigger_word; // 32 bits! :D
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
```

Intel 80186 (or i186 for short, or iAPX 186 for not really any short and kind of weird): 16 bit words, 16+8-bit addresses, wait what MOVING ON 8 bit characters, and you can do 32-bit math!
// AKA "This asshole..."

// TODO: bitch about near and far
```c
size_t size_sized_word; // 16 bits
intptr_t pointer_sized_word; // at least 24 bits, = 32 bits
int big_word; // 16 bits
// OR!!!!
int big_word; // 32 bits
long int bigger_word; // 32 bits!
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
```

Intel 80386: 32 bit words, 32 bit ~~or also 16+8-bit~~ ~~or also 16-bit addresses~~, 8 bit characters, you can do 64-bit math... this is getting tiresome, why is this happening
```c
size_t size_sized_word; // 32 bits
intptr_t pointer_sized_word; // 32 bits
int big_word; // 16 bits
// OR!!!!
int big_word; // 32 bits, more often but still not all the time GOD DAMMIT
long int bigger_word; // 32 bits!
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
```

(somewhere in there)
```c
// We can do 64 bit math and it doesn't suck, but long is already 32 bit, so
// THERE IS NO PROBLEM IF WE ADD:
long long int biggerer_word; // 64 bits! (usually)
```

SPARC!!! 32 bit words, 32 bit addresses, 8 bit characters, you can do maths
```c
int big_word; // 32 bits
long int bigger_word; // 64 bits
long long int biggerer_word; // 128 bits
short int not_as_little_word; 16 bits
char little_word; // 8 bits
```

skip literally two decades:

AMD Opteron: 64 bit words, 64 bit addresses, 8 bit characters, you can do ALL THE MATHS
```c
int big_word; // 32 bits because we're cool, but sometimes 16 bits because we're warm
long int bigger_word; // 32 bits
short int not_as_little_word; // 16 bits
char little_word; // 8 bits
long long int biggerer_word; // 64 bits
// I am kidding, thank goodness
//long long long int biggest_word; // 128 bits
```

Enter C99! ISO to the rescue! Let's have a standard:
```c
size_t size_sized_word; // we've had this for a while. big enough to hold the size of something. that big!
intptr_t pointer_sized_word; // Big enough to hold **any pointer**
int8_t littlest_word; // 8 bits!
int16_t little_word; // 16 bits!
int32_t regular_word; // 32 bits!
int64_t bigger_word; // 64 bits!
int128_t biggerer_word; // 128 bits!
intmax_t biggest_word; // What is the biggest word? That big
// wait, why'd it hilight that different?
// uh oh, I think I feel a STANDARD coming on!
int_least8_t littlest_word; // 8 bits!
int_least16_t little_word; // 16 bits!
int_least32_t regular_word; // 32 bits!
int_least64_t bigger_word; // 64 bits!
int_fast8_t littlest_word; // 8 bits!
int_fast16_t little_word; // 16 bits!
int_fast32_t regular_word; // 32 bits!
int_fast64_t bigger_word; // 64 bits!
```

PDP-10: 36 bit words, 18 bit addresses, 6 bit characters "PARIS "! :D
```c
//int8_t doesnt_exist;
//int16_t doesnt_exist;
//int32_t doesnt_exist;
//int64_t doesnt_exist;
// PLOT TWIST IT'S ALL MAYBES
int6_t tiny_word;
int9_t little_word;
int18_t half_word;
int36_t whole_word;
int72_t double_word;
int108_t triple_word; // maybe
int144_t quad_word; // also maybe
int_least8_t littlest_word; // 9 bits!
int_least16_t little_word; // 18 bits!
int_least32_t regular_word; // 36 bits!
int_least64_t bigger_word; // 72 bits!
int_fast8_t littlest_word; // 36 bits!
int_fast16_t little_word; // 36 bits!
int_fast32_t regular_word; // 36 bits!
int_fast64_t bigger_word; // 72 bits!
```

```c
int is_pointer;
intptr_t value;

// ...
if(is_pointer) {
    do_something(*(int*)value);
} else {
    do_something(value);
}
//printf("%" PRIdPTR "\n", value);
printf("%d\n", value);

// in a header somewhere:
typedef intptr_t int;
typedef roadhouse_t int;
#define PRIdPTR "d" // on systems where an int is big enough

#define PRIdPTR "ld" // on systems where a long int is big enough
#define PRIdPTR "lld" // on systems where a long long int is big enough

char patrick_swayze;
printf("%i\n", patrick_swayze);
short patrick_swayze;
printf("%i\n", patrick_swayze);
int patrick_swayze;
printf("%i\n", patrick_swayze);

printf("This is the same as");
printf("This is " "the" " same" " as");
```


```
7-bit ASCII:
1. 00-1F: control characters
2. 20-3F: punctuation
3. 40-5F: uppercase
4. 60-7F: lowercase (and also one control character because fuck you)

6-bit ASCII:
1. 00-1F: uppercase
2. 20-3F: punctuation
```


so you're on a modern system
```c
float f; // this is an IEEE 754 32-bit float
double d; // this is an IEEE 754 64-bit float
```

Intel 8087 floating point unit: 32-bit single-precision, 64-bit double-precision, 80-bit double-and-a-bit-precision. (Actually only has 80-bit, but shhhhhhh) 16-bit data bus
```c
float f; // so let's do 32-bit, because it's IEEE 754 compliant! :3
double d; // so let's do 64-bit, because IT'S IEEE 754 compliant! :3!!
long double ld; // ... I guess since we do sometimes need 80-bit precision and we already have long int let's just do this it'll be fine.
// SEEEEEEE_EEEEEEEE_1MMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM
```

Intel the other kind of floating point unit: 32-bit single-precision, 64-bit double-precision, 128-bit double-double-toil-and-trouble-precision. And it's actually IEEE this time. It's just all IEEE. And it actually has all of them this time. For real. this time.
```c
float f; // 32
double d; // 64
long double ld; // 128
```

Motorola 68881/68882: 32-bit single-precision, 64-bit double-precision, 80-bit double-and-a-bit-precision. 32-bit data bus
```c
float f; // 32-bit! :D
double d; // 64-bit! :DDDDD


// **NOTE:  Just don't use long double
//
long double ld; // 96-bit! WAIT WHAT THE FUCK—
// 00000000_00000000_SEEEEEEE_EEEEEEEE_1MMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM
// SEEEEEEE_EEEEEEEE_1MMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_00000000_00000000
// nope, it was:
// SEEEEEEE_EEEEEEEE_00000000_00000000_1MMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM
```

SPARC:
```c
 guys!
double d; // 64-bit! Oh man it's happening, Sun knows where it's at!
long double ld; // IT'S FUCKING 128 BIT! AND ALL THE BITS ARE REAL! AND IT'S IEEEE!!!!!!!!!!
// SEEEEEEE_EEEEEEEE_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMMfloat f; // 32-bit! I have a really good feelin
```

PowerPC:
```c
float f; // GUESS WHAT IT'S 32-bit!
double d; // HERE COMES 64-bit!
// SEEEEEEE_EEEEMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM

long double ld; // CAN I GET A 128-BIT IN THE HIZZOUSE!!!?!
// SEEEEEEE_EEEEMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_SEEEEEEE_EEEEMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM_MMMMMMMM
```


# Why doesn't Rust have globals?

Let's pretend it does:

```rust
let mut x = 5;

fn thread_a() {
    x = x + 1;
}

fn thread_b() {
    x = x - 1;
}
// 4, because thread_b started executing first, and finished executing second
// 6, because thread_a started executing first, and finished executing second
// 5, because one started and finished before the other started
// 4 or 5 or 6 because the caches fought each other and everyone lost
// corrupted horribly because oh no memory consistency is hard

fn main() {
    std::thread::spawn(thread_a);
    std::thread::spawn(thread_b);
}
```

```js
let x = 5;
function thread_a() {
    x = x + 1;
}
function thread_b() {
    x = x - 1;
}

thread_a();
thread_a();
thread_a();
thread_b();
thread_a();
thread_a();
thread_b();
thread_a();
thread_a();
```

It doesn't though

```rust
//  const: Can't ever change. Known, entirely, at compile time. Doesn't exist.
const GLOBAL_CONSTANT: i32 = 1347689;
// static: Can't ever change. Known, entirely, at compile time. Exists.
static GLOBAL_STATIC: i32 = 1347689;

fn main() {
    println!("global constant is: {}", GLOBAL_CONSTANT);
    println!("global static is: {}", GLOBAL_STATIC);
    // exactly as if we had written:
    println!("global constant is: {}", 1347689);
    println!("global static is: {}", GLOBAL_STATIC);
    // but what if:
    println!("global constant is: {}", &GLOBAL_CONSTANT);
    println!("global static is: {}", &GLOBAL_STATIC);
    // as if:
    println!("global constant is: {}", &1347689);
    println!("global static is: {}", &GLOBAL_STATIC);
}
```

```rust
static mut GLOBAL_DINGUS: LeafletCounter = LeafletCounter::new();

fn main() {
    unsafe {
        GLOBAL_DINGUS.propaganda_leaflets += 1;
        GLOBAL_DINGUS.intelligence_recruitment_leaflets += 10;
    }
}
```

```rust
static mut GLOBAL_DINGUS: LeafletCounter = LeafletCounter::new();

fn campaign_a() {
    unsafe {
        GLOBAL_DINGUS.propaganda_leaflets += 1;
        GLOBAL_DINGUS.intelligence_recruitment_leaflets += 10;
    }
}

fn campaign_b() {
    unsafe {
        GLOBAL_DINGUS.propaganda_leaflets += 99;
        GLOBAL_DINGUS.intelligence_recruitment_leaflets += 1;
    }
}

fn main() {
    std::thread::spawn(campaign_a);
    std::thread::spawn(campaign_b);
}
```

```rust
// static: Can't ever change. Known, entirely, at compile time. Exists.
// Except...
// Mutex = MUTual EXclusion
static GLOBAL_DINGUS: Mutex<LeafletCounter> = Mutex::new(LeafletCounter::new());
// Whetting appetite and/or causing fear, there are lots of other ways to get
// interior mutability, with their own tradeoffs. Small sampling:
// RwLock, OnceCell, OnceLock, RefCell, UnsafeCell, Cell

fn campaign_a() {
    let locked_dingus = GLOBAL_DINGUS.lock();
    locked_dingus.propaganda_leaflets += 1;
    locked_dingus.intelligence_recruitment_leaflets += 10;
}

fn campaign_b() {
    let locked_dingus = GLOBAL_DINGUS.lock();
    locked_dingus.propaganda_leaflets += 99;
    locked_dingus.intelligence_recruitment_leaflets += 1;
}

fn main() {
    // Can't do this:
    //GLOBAL_DINGUS = something_else;
    // Can't do this:
    //GLOBAL_DINGUS.do_something_that_requires_mut();
    std::thread::spawn(campaign_a);
    std::thread::spawn(campaign_b);
}
```

# NOW DO: bitch about near and far (and huge)


```c
MySuperSecretStruct* foo = (MySuperSecretStruct*)malloc(sizeof(MySuperSecretStruct));
```
```

cvoid* my_pointer; // How big is this?
// 32-bit machine: 32-bit
// 64-bit machine: 64-bit
// 128-bit machine: 128-bitc```

```c
/// 
// The dark times (before the furry thing touched us and freaked us out)  uhh
 8086
void* my_pointer; // 16 bits


// 6502
void* my_pointer: // 16 bits
```

```c
// And now everybody's going big!
// 80186: 8086 but with more addresses
void* my_pointer; // still has to be 16 bits, for backwards compatibility
void near* my_pointer; // 16 bits
void far* my_pointer: // 16 bits + segment number = 24 bits
void huge* my_pointer; // 24 bits = 24 bits?

// 65816: 6502 but with more of many things
void* my_pointer; // still has to be 16 bits
void near* my_pointer; // 16 bits
void far* my_pointer; // 24 bits

// Everyone else:
// holy shit this is a nightmare let's just do linear addressing jesus christ

// Some asshole Digital Signal Processor vendor:
// let's have two kinds of memory now that we can distinguish them with near and far!
void near* my_pointer: // a pointer to only one kind of memory, 16 bits
void far* my_pointer; // a pointer that can tell you what kind of memory it's pointing to but it's also 16 bits because DSP is its own nightmare rabbit hole AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
```

# bad error messages

```c
int some_external_flag;
int main() {
    switch(some_external_flag) {
    case 0: printf("It was zero!\n"); break;
    case 1: printf("It was one!\n"); break;
    default:
        if(some_external_flag % 2 == 0) goto was_even;
        printf("Guess it was odd\n");
        break;
    was_even:
        printf("Guess it was even\n");
    }
}
```

GCC will say something like:
```
yourfile.c:10: error: labels aren't allowed in a switch (-ftraditional)
```

MPW C will say something like:
```
...And the lord said, 'lo, there shall only be case or default labels inside a switch statement'
```

an IBM C compiler from ~1980 would say something like:
```
Error 13
```

## more MPW messages:

```
- ```
- Too many errors on one line (make fewer)
- Symbol table full - fatal heap error; please go buy a RAM upgrade from your local Apple dealer
- type in (cast) must be scalar; ANSI 3.3.4; page 39, lines 10-11 (I know you don't care, I'm just trying to annoy you)
- a typedef name was a complete surprise to me at this point in your program
- String literal too long (I let you have 512 characters, that's 3 more than ANSI said I should)
- Call me paranoid but finding '/*' inside this comment makes me suspicious

and many more
```c
/*
printf("Some code I want to comment out\n");
/* Here is a comment */
printf("Some code I want to comment out\n");
*/
```
