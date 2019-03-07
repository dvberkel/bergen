module Main exposing (main)

import BrnFck exposing (Machine, Message, machine, view, update, subscriptions)
import Browser
import Css exposing (..)
import Html as PlainHtml
import Html.Styled as Html exposing (Html, toUnstyled)


main : Program () Machine Message
main =
    Browser.element
        { init = \_ -> (init, Cmd.none)
        , view = view >> toUnstyled
        , update = update
        , subscriptions = subscriptions
        }


init : Machine
init =
    machine 5
