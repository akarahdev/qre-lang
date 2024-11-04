# QIR
QIR is an intermediate language for QRE.

Example:

In QRE:
```qre
struct list[T] {
    // ...
}

fn list::append[T](l: list[T], val: T) {
    // ...
}

struct X {
    box: i32;
}

struct Y {
    box: i64;
}

fn main() -> i32 {
    let v: X | Y = init X;
    match v {
        struct X -> {
            v->box = 5;
        }
        struct Y -> {
            v->box = 10;
        }
    }
    
    let li: list[X | Y] = init list;
    li->append(v);
}
```

In QIR:
```qir
struct list::@XorY {
    type_id: 0i8;
    // ...
}

fn list::@XorY::append(l: list::@XorY, v: ptr) -> void {
    // ...
}

struct X { type_id: 1i8; ref_count: i16; box: i32; }
struct Y { type_id: 2i8; ref_count: i16; box: i32; }

fn main() -> i32 {
    let v: ptr = init x;
    retain v;
    // ->0 gets the first field, aka `type_id`
    // ->1 is the second field for the refcount
    // ->2 and beyond are struct-specific
    if v->0 == 1 {
        v->1 = 5;
    } else if v->0 == 2 {
        v->2 = 10;
    } else {
        hint::unreachable();
    }
   
    let li: ptr = zeroinit list::@XorY;
    retain li;
    list::@XorY::append(li);
    
    release li;
    release v;
}
```