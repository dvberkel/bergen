module BrnFck exposing (Machine, decrement, decrementPointer, increment, incrementPointer, machine, pointerAt, valueAt)

import Array exposing (Array)


type Machine
    = Machine MachineState


type alias MachineState =
    { pointer : Int
    , size : Int
    , registers : Array Int
    }


machine : Int -> Machine
machine size =
    Machine { pointer = 0, size = size, registers = Array.repeat size 0 }


incrementPointer : Machine -> Machine
incrementPointer (Machine ({ pointer, size } as state)) =
    let
        value =
            min (size - 1) (pointer + 1)
    in
    Machine { state | pointer = value }


decrementPointer : Machine -> Machine
decrementPointer (Machine ({ pointer } as state)) =
    let
        value =
            max 0 (pointer - 1)
    in
    Machine { state | pointer = value }


pointerAt : Int -> Machine -> Machine
pointerAt pointer (Machine state) =
    Machine { state | pointer = pointer }


increment : Machine -> Machine
increment (Machine ({ registers, pointer } as state)) =
    let
        value =
            registers
                |> Array.get pointer
                |> Maybe.withDefault 0
                |> inc
    in
    Machine { state | registers = registers |> Array.set pointer value }


inc : Int -> Int
inc n =
    n + 1


decrement : Machine -> Machine
decrement (Machine ({ registers, pointer } as state)) =
    let
        value =
            registers
                |> Array.get pointer
                |> Maybe.withDefault 0
                |> dec
    in
    Machine { state | registers = registers |> Array.set pointer (max 0 value) }


dec : Int -> Int
dec n =
    n - 1


valueAt : Int -> Int -> Machine -> Machine
valueAt pointer value (Machine ({ registers } as state)) =
    Machine { state | registers = registers |> Array.set pointer value }
