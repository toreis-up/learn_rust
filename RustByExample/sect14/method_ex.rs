// 具象型S
struct S;

// Generic型GenericVal
struct GenericVal<T>(T);

// 型パラメータを指定した上で、GenericValにメソッドを実装
impl GenericVal<f32> {}

// Sを指定し、GenericVal<S>にメソッドを実装
impl GenericVal<S> {}

// ジェネリック型のまま扱うには<T>が先に来る必要がある
impl<T> GenericVal<T> {}