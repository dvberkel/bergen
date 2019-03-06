module Main exposing (main)

import BrnFck exposing (Machine, machine, view, update)
import Browser
import Css exposing (..)
import Html as PlainHtml
import Html.Styled as Html exposing (Html, toUnstyled)


main =
    Browser.sandbox
        { init = init
        , view = view >> toUnstyled
        , update = update
        }


init : Machine
init =
    machine 5
