module BrnFck exposing (Machine, decrement, decrementPointer, increment, incrementPointer, machine, pointerAt, valueAt, view)

import Array exposing (Array)
import Css exposing (..)
import Html as PlainHtml
import Html.Styled as Html exposing (Html, toUnstyled)
import Html.Styled.Attributes as Attribute


type Machine
    = Machine MachineState

type alias Register = Int


type alias MachineState =
    { pointer : Int
    , size : Int
    , registers : Array Register
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


inc : Register -> Register
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


dec : Register -> Register
dec n =
    n - 1


valueAt : Int -> Register -> Machine -> Machine
valueAt pointer value (Machine ({ registers } as state)) =
    Machine { state | registers = registers |> Array.set pointer value }

view : Machine -> Html msg
view aMachine =
    Html.div [ Attribute.class "machine"] [
         viewRegisters aMachine
        ]

viewRegisters : Machine -> Html msg
viewRegisters (Machine {registers, pointer}) =
    let
        viewOfRegisters =
            registers
            |> Array.indexedMap (viewRegister pointer)
            |> Array.toList
    in
    Html.div [Attribute.class "registers"]
        viewOfRegisters

viewRegister : Int -> Int -> Register -> Html msg
viewRegister pointer index register =
    let
        label = String.fromInt register
    in
    Html.span [ Attribute.class "register"] [Html.text label]
