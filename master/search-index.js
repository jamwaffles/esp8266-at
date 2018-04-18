var searchIndex = {};
searchIndex["embedded_hal"] = {"doc":"A Hardware Abstraction Layer (HAL) for embedded systems","items":[[4,"Direction","embedded_hal","Count direction",null,null],[13,"Downcounting","","3, 2, 1",0,null],[13,"Upcounting","","1, 2, 3",0,null],[0,"blocking","","Blocking API",null,null],[0,"delay","embedded_hal::blocking","Delays",null,null],[8,"DelayMs","embedded_hal::blocking::delay","Millisecond delay",null,null],[10,"delay_ms","","Pauses execution for `ms` milliseconds",1,{"inputs":[{"name":"self"},{"name":"uxx"}],"output":null}],[8,"DelayUs","","Microsecond delay",null,null],[10,"delay_us","","Pauses execution for `us` microseconds",2,{"inputs":[{"name":"self"},{"name":"uxx"}],"output":null}],[0,"i2c","embedded_hal::blocking","Blocking I2C API",null,null],[8,"Read","embedded_hal::blocking::i2c","Blocking read",null,null],[16,"Error","","Error type",3,null],[10,"read","","Reads enough bytes from slave with `address` to fill `buffer`",3,null],[8,"Write","","Blocking write",null,null],[16,"Error","","Error type",4,null],[10,"write","","Sends bytes to slave with address `addr`",4,null],[8,"WriteRead","","Blocking write + read",null,null],[16,"Error","","Error type",5,null],[10,"write_read","","Sends bytes to slave with address `addr` and then reads enough bytes to fill `buffer` in a single transaction",5,null],[0,"serial","embedded_hal::blocking","Blocking serial API",null,null],[0,"write","embedded_hal::blocking::serial","Blocking serial write",null,null],[8,"Default","embedded_hal::blocking::serial::write","Marker trait to opt into default blocking write implementation",null,null],[8,"Write","embedded_hal::blocking::serial","Write half of a serial interface (blocking variant)",null,null],[16,"Error","","The type of error that can occur when writing",6,null],[10,"bwrite_all","","Writes a slice, blocking until everything has been written",6,null],[10,"bflush","","Block until the serial interface has sent all buffered words",6,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[0,"spi","embedded_hal::blocking","Blocking SPI API",null,null],[0,"transfer","embedded_hal::blocking::spi","Blocking transfer",null,null],[8,"Default","embedded_hal::blocking::spi::transfer","Default implementation of `blocking::spi::Transfer<W>` for implementers of `spi::FullDuplex<W>`",null,null],[0,"write","embedded_hal::blocking::spi","Blocking write",null,null],[8,"Default","embedded_hal::blocking::spi::write","Default implementation of `blocking::spi::Write<W>` for implementers of `spi::FullDuplex<W>`",null,null],[8,"Transfer","embedded_hal::blocking::spi","Blocking transfer",null,null],[16,"Error","","Error type",7,null],[10,"transfer","","Sends `words` to the slave. Returns the `words` received from the slave",7,null],[8,"Write","","Blocking write",null,null],[16,"Error","","Error type",8,null],[10,"write","","Sends `words` to the slave, ignoring all the incoming words",8,null],[0,"digital","embedded_hal","Digital I/O",null,null],[8,"OutputPin","embedded_hal::digital","Single digital output pin",null,null],[10,"is_high","","Is the output pin high?",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[10,"is_low","","Is the output pin low?",9,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[10,"set_low","","Sets the pin low",9,{"inputs":[{"name":"self"}],"output":null}],[10,"set_high","","Sets the pin high",9,{"inputs":[{"name":"self"}],"output":null}],[8,"InputPin","","Single digital input pin",null,null],[10,"is_high","","Is the input pin high?",10,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[10,"is_low","","Is the input pin low?",10,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[0,"prelude","embedded_hal","The prelude is a collection of all the traits in this crate",null,null],[0,"serial","","Serial interface",null,null],[8,"Read","embedded_hal::serial","Read half of a serial interface",null,null],[16,"Error","","Read error",11,null],[10,"read","","Reads a single word from the serial interface",11,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[8,"Write","","Write half of a serial interface",null,null],[16,"Error","","Write error",12,null],[10,"write","","Writes a single word to the serial interface",12,{"inputs":[{"name":"self"},{"name":"word"}],"output":{"name":"result"}}],[10,"flush","","Ensures that none of the previously written words are still buffered",12,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[0,"spi","embedded_hal","Serial Peripheral Interface",null,null],[3,"Mode","embedded_hal::spi","SPI mode",null,null],[12,"polarity","","Clock polarity",13,null],[12,"phase","","Clock phase",13,null],[4,"Polarity","","Clock polarity",null,null],[13,"IdleLow","","Clock signal low when idle",14,null],[13,"IdleHigh","","Clock signal high when idle",14,null],[4,"Phase","","Clock phase",null,null],[13,"CaptureOnFirstTransition","","Data in \"captured\" on the first clock transition",15,null],[13,"CaptureOnSecondTransition","","Data in \"captured\" on the second clock transition",15,null],[8,"FullDuplex","","Full duplex (master mode)",null,null],[16,"Error","","An enumeration of SPI errors",16,null],[10,"read","","Reads the word stored in the shift register",16,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[10,"send","","Sends a word to the slave",16,{"inputs":[{"name":"self"},{"name":"word"}],"output":{"name":"result"}}],[11,"clone","","",14,{"inputs":[{"name":"self"}],"output":{"name":"polarity"}}],[11,"eq","","",14,{"inputs":[{"name":"self"},{"name":"polarity"}],"output":{"name":"bool"}}],[11,"clone","","",15,{"inputs":[{"name":"self"}],"output":{"name":"phase"}}],[11,"eq","","",15,{"inputs":[{"name":"self"},{"name":"phase"}],"output":{"name":"bool"}}],[11,"clone","","",13,{"inputs":[{"name":"self"}],"output":{"name":"mode"}}],[11,"eq","","",13,{"inputs":[{"name":"self"},{"name":"mode"}],"output":{"name":"bool"}}],[11,"ne","","",13,{"inputs":[{"name":"self"},{"name":"mode"}],"output":{"name":"bool"}}],[0,"timer","embedded_hal","Timers",null,null],[8,"CountDown","embedded_hal::timer","A count down timer",null,null],[16,"Time","","The unit of time used by this timer",17,null],[10,"start","","Starts a new count down",17,{"inputs":[{"name":"self"},{"name":"t"}],"output":null}],[10,"wait","","Non-blockingly \"waits\" until the count down finishes",17,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[8,"Periodic","","Marker trait that indicates that a timer is periodic",null,null],[8,"Capture","embedded_hal","Input capture",null,null],[16,"Error","","Enumeration of `Capture` errors",18,null],[16,"Channel","","Enumeration of channels that can be used with this `Capture` interface",18,null],[16,"Time","","A time unit that can be converted into a human time unit (e.g. seconds)",18,null],[16,"Capture","","The type of the value returned by `capture`",18,null],[10,"capture","","\"Waits\" for a transition in the capture `channel` and returns the value of counter at that instant",18,null],[10,"disable","","Disables a capture `channel`",18,null],[10,"enable","","Enables a capture `channel`",18,null],[10,"get_resolution","","Returns the current resolution",18,null],[10,"set_resolution","","Sets the resolution of the capture timer",18,{"inputs":[{"name":"self"},{"name":"r"}],"output":null}],[8,"Pwm","","Pulse Width Modulation",null,null],[16,"Channel","","Enumeration of channels that can be used with this `Pwm` interface",19,null],[16,"Time","","A time unit that can be converted into a human time unit (e.g. seconds)",19,null],[16,"Duty","","Type for the `duty` methods",19,null],[10,"disable","","Disables a PWM `channel`",19,null],[10,"enable","","Enables a PWM `channel`",19,null],[10,"get_period","","Returns the current PWM period",19,null],[10,"get_duty","","Returns the current duty cycle",19,null],[10,"get_max_duty","","Returns the maximum duty cycle value",19,null],[10,"set_duty","","Sets a new duty cycle",19,null],[10,"set_period","","Sets a new PWM period",19,{"inputs":[{"name":"self"},{"name":"p"}],"output":null}],[8,"PwmPin","","A single PWM channel / pin",null,null],[16,"Duty","","Type for the `duty` methods",20,null],[10,"disable","","Disables a PWM `channel`",20,{"inputs":[{"name":"self"}],"output":null}],[10,"enable","","Enables a PWM `channel`",20,{"inputs":[{"name":"self"}],"output":null}],[10,"get_duty","","Returns the current duty cycle",20,null],[10,"get_max_duty","","Returns the maximum duty cycle value",20,null],[10,"set_duty","","Sets a new duty cycle",20,null],[8,"Qei","","Quadrature encoder interface",null,null],[16,"Count","","The type of the value returned by `count`",21,null],[10,"count","","Returns the current pulse count of the encoder",21,null],[10,"direction","","Returns the count direction",21,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"direction"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"direction"}],"output":{"name":"bool"}}]],"paths":[[4,"Direction"],[8,"DelayMs"],[8,"DelayUs"],[8,"Read"],[8,"Write"],[8,"WriteRead"],[8,"Write"],[8,"Transfer"],[8,"Write"],[8,"OutputPin"],[8,"InputPin"],[8,"Read"],[8,"Write"],[3,"Mode"],[4,"Polarity"],[4,"Phase"],[8,"FullDuplex"],[8,"CountDown"],[8,"Capture"],[8,"Pwm"],[8,"PwmPin"],[8,"Qei"]]};
searchIndex["esp8266_at"] = {"doc":"","items":[[3,"ESP8266","esp8266_at","",null,null],[4,"State","","",null,null],[13,"AtTest","","",0,null],[13,"Connected","","",0,null],[13,"Connecting","","",0,null],[13,"Disconnected","","",0,null],[13,"EnableDhcp","","",0,null],[13,"Reset","","",0,null],[13,"Resetting","","",0,null],[13,"SetMode","","",0,null],[13,"Uninitialized","","",0,null],[13,"WifiGotIp","","",0,null],[13,"WifiReady","","",0,null],[13,"ConnectionFailed","","",0,null],[13,"Waiting","","",0,null],[13,"SetAccessPoint","","",0,null],[13,"ConnectedToAp","","",0,null],[13,"SetAp","","",0,null],[4,"Command","","",null,null],[13,"Reset","","",1,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"state"}}],[11,"new","","",2,{"inputs":[],"output":{"name":"self"}}],[11,"statemachine","","",2,{"inputs":[{"name":"self"}],"output":null}],[11,"next_command","","",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"handle_bytes","","",2,null]],"paths":[[4,"State"],[4,"Command"],[3,"ESP8266"]]};
searchIndex["heapless"] = {"doc":"`static` friendly data structures that don't require dynamic memory allocation","items":[[3,"LinearMap","heapless","A map / dictionary backed by an array that performs lookups via linear search",null,null],[3,"String","","A String backed by a fixed size `heapless::Vec`",null,null],[3,"Vec","","A [`Vec`], vector, backed by a fixed size array",null,null],[3,"BufferFullError","","Error raised when the buffer is full",null,null],[11,"new","","Creates an empty `LinearMap`",0,{"inputs":[],"output":{"name":"self"}}],[11,"capacity","","Returns the number of elements that the map can hold",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"clear","","Clears the map, removing all key-value pairs",0,{"inputs":[{"name":"self"}],"output":null}],[11,"contains_key","","Returns true if the map contains a value for the specified key.",0,{"inputs":[{"name":"self"},{"name":"k"}],"output":{"name":"bool"}}],[11,"get","","Returns a reference to the value corresponding to the key",0,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"get_mut","","Returns a mutable reference to the value corresponding to the key",0,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"len","","Returns the number of elements in this map",0,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"insert","","Inserts a key-value pair into the map.",0,{"inputs":[{"name":"self"},{"name":"k"},{"name":"v"}],"output":{"generics":["option","bufferfullerror"],"name":"result"}}],[11,"is_empty","","Returns true if the map contains no elements",0,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"iter","","An iterator visiting all key-value pairs in arbitrary order.",0,{"inputs":[{"name":"self"}],"output":{"name":"iter"}}],[11,"iter_mut","","An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values",0,{"inputs":[{"name":"self"}],"output":{"name":"itermut"}}],[11,"keys","","An iterator visiting all keys in arbitrary order",0,null],[11,"remove","","Removes a key from the map, returning the value at the key if the key was previously in the map",0,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"option"}}],[11,"values","","An iterator visiting all values in arbitrary order",0,null],[11,"values_mut","","An iterator visiting all values mutably in arbitrary order",0,null],[11,"index","","",0,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"v"}}],[11,"index_mut","","",0,{"inputs":[{"name":"self"},{"name":"q"}],"output":{"name":"v"}}],[0,"ring_buffer","","Ring buffer",null,null],[3,"Consumer","heapless::ring_buffer","A ring buffer \"consumer\"; it can dequeue items from the ring buffer",null,null],[3,"Producer","","A ring buffer \"producer\"; it can enqueue items into the ring buffer",null,null],[3,"RingBuffer","","An statically allocated ring buffer backed by an array `A`",null,null],[3,"Iter","","An iterator over a ring buffer items",null,null],[3,"IterMut","","A mutable iterator over a ring buffer items",null,null],[11,"split","","Splits a statically allocated ring buffer into producer and consumer end points",1,null],[11,"dequeue","","Returns the item in the front of the queue, or `None` if the queue is empty",2,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"enqueue","","Adds an `item` to the end of the queue",3,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"new","","Creates an empty ring buffer with capacity equals to the length of the array `A` minus one.",1,{"inputs":[],"output":{"name":"self"}}],[11,"capacity","","Returns the maximum number of elements the ring buffer can hold",1,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"dequeue","","Returns the item in the front of the queue, or `None` if the queue is empty",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"enqueue","","Adds an `item` to the end of the queue",1,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"len","","Returns the number of elements in the queue",1,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"is_empty","","Returns `true` if the ring buffer has a length of 0",1,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"iter","","Iterates from the front of the queue to the back",1,{"inputs":[{"name":"self"}],"output":{"name":"iter"}}],[11,"iter_mut","","Returns an iterator that allows modifying each value.",1,{"inputs":[{"name":"self"}],"output":{"name":"itermut"}}],[11,"drop","","",1,{"inputs":[{"name":"self"}],"output":null}],[11,"next","","",4,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",5,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"new","heapless","Constructs a new, empty `String`",6,{"inputs":[],"output":{"name":"self"}}],[11,"from_utf8","","Converts a vector of bytes into a `String`.",6,{"inputs":[{"generics":["u8"],"name":"vec"}],"output":{"generics":["string","utf8error"],"name":"result"}}],[11,"from_utf8_unchecked","","Converts a vector of bytes to a `String` without checking that the string contains valid UTF-8.",6,{"inputs":[{"generics":["u8"],"name":"vec"}],"output":{"name":"string"}}],[11,"into_bytes","","Converts a `String` into a byte vector.",6,{"inputs":[{"name":"self"}],"output":{"generics":["u8"],"name":"vec"}}],[11,"as_str","","Extracts a string slice containing the entire string.",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"as_mut_str","","Converts a `String` into a mutable string slice.",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"push_str","","Appends a given string slice onto the end of this `String`.",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"capacity","","Returns the maximum number of elements the String can hold",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"push","","Appends the given [`char`] to the end of this `String`.",6,{"inputs":[{"name":"self"},{"name":"char"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"as_bytes","","Returns a byte slice of this `String`'s contents.",6,null],[11,"truncate","","Shortens this `String` to the specified length.",6,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"pop","","Removes the last character from the string buffer and returns it.",6,{"inputs":[{"name":"self"}],"output":{"generics":["char"],"name":"option"}}],[11,"is_empty","","Returns `true` if this `String` has a length of zero.",6,{"inputs":[{"name":"self"}],"output":{"name":"bool"}}],[11,"clear","","Truncates this `String`, removing all contents.",6,{"inputs":[{"name":"self"}],"output":null}],[11,"len","","Returns the length of this `String`, in bytes.",6,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"from","","",6,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"fmt","","",6,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"write_str","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"generics":["error"],"name":"result"}}],[11,"write_char","","",6,{"inputs":[{"name":"self"},{"name":"char"}],"output":{"generics":["error"],"name":"result"}}],[11,"deref","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"deref_mut","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"as_ref","","",6,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"as_ref","","",6,null],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"string"}],"output":{"name":"bool"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"eq","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"bool"}}],[11,"new","","Constructs a new, empty `Vec<T>` backed by the array `A`",7,{"inputs":[],"output":{"name":"self"}}],[11,"capacity","","Returns the maximum number of elements the vector can hold",7,{"inputs":[{"name":"self"}],"output":{"name":"usize"}}],[11,"clear","","Clears the vector, removing all values.",7,{"inputs":[{"name":"self"}],"output":null}],[11,"extend_from_slice","","Clones and appends all elements in a slice to the `Vec`.",7,null],[11,"pop","","Removes the last element from a vector and return it, or `None` if it's empty",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"push","","Appends an element to the back of the collection",7,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"truncate","","Shortens the vector, keeping the first `len` elements and dropping the rest.",7,{"inputs":[{"name":"self"},{"name":"usize"}],"output":null}],[11,"resize","","Resizes the Vec in-place so that len is equal to new_len.",7,{"inputs":[{"name":"self"},{"name":"usize"},{"name":"t"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"resize_default","","Resizes the `Vec` in-place so that `len` is equal to `new_len`.",7,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"generics":["bufferfullerror"],"name":"result"}}],[11,"swap_remove","","Removes an element from the vector and returns it.",7,{"inputs":[{"name":"self"},{"name":"usize"}],"output":{"name":"t"}}],[11,"fmt","","",7,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",7,{"inputs":[{"name":"self"}],"output":null}],[11,"eq","","",7,{"inputs":[{"name":"self"},{"name":"vec"}],"output":{"name":"bool"}}],[11,"deref","","",7,null],[11,"deref_mut","","",7,null],[11,"as_ref","","",7,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"as_mut","","",7,{"inputs":[{"name":"self"}],"output":{"name":"vec"}}],[11,"as_ref","","",7,null],[11,"as_mut","","",7,null],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"bufferfullerror"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",8,{"inputs":[{"name":"self"},{"name":"bufferfullerror"}],"output":{"name":"bool"}}]],"paths":[[3,"LinearMap"],[3,"RingBuffer"],[3,"Consumer"],[3,"Producer"],[3,"Iter"],[3,"IterMut"],[3,"String"],[3,"Vec"],[3,"BufferFullError"]]};
searchIndex["nb"] = {"doc":"Minimal and reusable non-blocking I/O layer","items":[[4,"Error","nb","A non-blocking error",null,null],[13,"Other","","A different kind of error",0,null],[13,"WouldBlock","","This operation requires blocking behavior to complete",0,null],[6,"Result","","A non-blocking result",null,null],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"error"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"generics":["ordering"],"name":"option"}}],[11,"lt","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"le","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"gt","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"ge","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"cmp","","",0,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"ordering"}}],[11,"hash","","",0,null],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[14,"await","","Await operation (won't work until the language gains support for generators)",null,null],[14,"block","","Turns the non-blocking expression `$e` into a blocking operation.",null,null],[14,"try_nb","","Future adapter",null,null]],"paths":[[4,"Error"]]};
searchIndex["untagged_option"] = {"doc":"Provides an unsafe tagless alternative to `Option<T>` that uses less memory.","items":[[19,"UntaggedOption","untagged_option","A union which either holds a `T` or nothing.",null,null],[12,"some","","",0,null],[12,"none","","",0,null],[11,"none","","Creates a new `UntaggedOption` holding no value.",0,{"inputs":[],"output":{"name":"self"}}],[11,"some","","Creates an `UntaggedOption` containing `t`.",0,{"inputs":[{"name":"t"}],"output":{"name":"self"}}],[11,"take","","Takes the `T` out of an initialized wrapper, making it uninitialized.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"as_ref","","Obtains an immutable reference to the contained `T`.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"as_mut","","Obtains a mutable reference to the contained `T`.",0,{"inputs":[{"name":"self"}],"output":{"name":"t"}}]],"paths":[[19,"UntaggedOption"]]};
initSearch(searchIndex);