(let a (C#4:1.5))
(let b (C#4:1.5 D#4:1.5 F#4:1.5 Cbb4:1.5))

(fn f (freq)
    (+ (sint freq) (cost freq)))

(fn f (a b)
    (+ a b))

(out ((f a) (f b))
     f a)

(def a (+ 1 2))
(def func (a b) (+ a b))

"a",
SymbolDef {
    args: HashMap::from([]),
    eval: (+ 1 2)
}

"func", 
SymbolDef {
    args: HashMap::from([(0, "a"), (1, "b")]),
    eval: (+ Arg(0) Arg(1))
}
