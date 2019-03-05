module Main exposing (main)

import BrnFck exposing (Machine, machine)
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


view : Machine -> Html msg
view _ =
    Html.div [] [ Html.span [] [ Html.text "Hello, World" ] ]


update : msg -> Machine -> Machine
update _ machine =
    machine
