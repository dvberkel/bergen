module BrnFck exposing (Machine, Message, decrement, decrementPointer, increment, incrementPointer, machine, output, pointerAt, subscriptions, update, valueAt, view, withOutput)

import Array exposing (Array)
import Css exposing (..)
import Html as PlainHtml
import Html.Styled as Html exposing (Html, toUnstyled)
import Html.Styled.Attributes as Attribute exposing (css)
import Html.Styled.Events as Event
import Keyboard exposing (RawKey)


type Machine
    = Machine MachineState


type alias Register =
    Int


type alias Pointer =
    Int


type alias MachineState =
    { pointer : Pointer
    , size : Int
    , registers : Array Register
    , stdout : String
    , readyToReceive : Bool
    }


machine : Int -> Machine
machine size =
    Machine
        { pointer = 0
        , size = size
        , registers = Array.repeat size 0
        , stdout = ""
        , readyToReceive = False
        }


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


pointerAt : Pointer -> Machine -> Machine
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


output : Machine -> Machine
output (Machine ({ registers, pointer, stdout } as state)) =
    let
        character =
            registers
                |> Array.get pointer
                |> Maybe.withDefault 0
                |> Char.fromCode
                |> String.fromChar
    in
    Machine { state | stdout = String.append stdout character }


withOutput : String -> Machine -> Machine
withOutput anOutput (Machine state) =
    Machine { state | stdout = anOutput }


stdoutOf : Machine -> String
stdoutOf (Machine { stdout }) =
    stdout


dec : Register -> Register
dec n =
    n - 1


startReceiving : Machine -> Machine
startReceiving (Machine state) =
    Machine { state | readyToReceive = True }

stopReceiving (Machine state) =
    Machine { state | readyToReceive = False }

input : Register -> Machine -> Machine
input value ((Machine {pointer}) as aMachine) =
    valueAt pointer value aMachine

valueAt : Pointer -> Register -> Machine -> Machine
valueAt pointer value (Machine ({ registers } as state)) =
    Machine { state | registers = registers |> Array.set pointer value }


view : Machine -> Html Message
view aMachine =
    Html.div [ Attribute.class "machine" ]
        [ viewRegisters aMachine
        , viewControls
        , viewOutput aMachine
        ]


viewRegisters : Machine -> Html msg
viewRegisters (Machine { registers, pointer }) =
    let
        viewOfRegisters =
            registers
                |> Array.indexedMap (viewRegister pointer)
                |> Array.toList
    in
    Html.div [ Attribute.class "registers" ]
        viewOfRegisters


viewRegister : Pointer -> Int -> Register -> Html msg
viewRegister pointer index register =
    let
        label =
            String.fromInt register

        colorOfBackground =
            if pointer == index then
                black

            else
                white

        colorOfText =
            if pointer == index then
                white

            else
                black
    in
    Html.span
        [ Attribute.class "register"
        , css
            [ display inlineBlock
            , borderColor black
            , borderStyle solid
            , borderWidth (px 1)
            , width (em 3)
            , height (ex 3)
            , lineHeight (ex 3)
            , textAlign center
            , backgroundColor colorOfBackground
            , color colorOfText
            , margin (px 2)
            ]
        ]
        [ Html.text label ]


black : Color
black =
    rgb 0 0 0


white : Color
white =
    rgb 255 255 255


viewControls : Html Message
viewControls =
    Html.div [ Attribute.class "controls" ]
        [ control "<" DecrementPointer
        , control ">" IncrementPointer
        , control "-" Decrement
        , control "+" Increment
        , control "." Output
        , control "," Input
        , control "clear" Clear
        ]


control : String -> msg -> Html msg
control label onClickMessage =
    Html.button [ Attribute.class "control", Event.onClick onClickMessage ] [ Html.text label ]


viewOutput : Machine -> Html msg
viewOutput aMachine =
    let
        stdout =
            stdoutOf aMachine
    in
    Html.pre [ Attribute.class "stdout" ] [ Html.text stdout ]


type Message
    = IncrementPointer
    | DecrementPointer
    | Increment
    | Decrement
    | Output
    | Input
    | KeyDown RawKey
    | KeyUp RawKey
    | Clear


update : Message -> Machine -> ( Machine, Cmd Message )
update message ((Machine { readyToReceive }) as aMachine) =
    let
        nextMachine =
            case message of
                IncrementPointer ->
                    incrementPointer aMachine

                DecrementPointer ->
                    decrementPointer aMachine

                Increment ->
                    increment aMachine

                Decrement ->
                    decrement aMachine

                Output ->
                    output aMachine

                Input ->
                    startReceiving aMachine

                KeyDown aKey ->
                    if readyToReceive then
                        let
                            value =
                                aKey
                                    |> Keyboard.rawValue
                                    |> String.toList
                                    |> List.head
                                    |> Maybe.map Char.toCode
                                    |> Maybe.withDefault 0
                        in
                            input value aMachine
                    else
                        aMachine

                KeyUp _ ->
                    stopReceiving aMachine

                Clear ->
                    aMachine
                        |> withOutput ""
    in
    ( nextMachine, Cmd.none )


subscriptions : Machine -> Sub Message
subscriptions aMachine =
    Sub.batch
        [ Keyboard.downs KeyDown
        , Keyboard.ups KeyUp
        ]
