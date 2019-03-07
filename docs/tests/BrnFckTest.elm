module BrnFckTest exposing (suite)

import BrnFck exposing (..)
import Expect exposing (Expectation)
import Test exposing (..)


suite : Test
suite =
    describe "BrnFck"
        [ describe "Machine"
            [ describe "operations"
                [ test "incrementPointer" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> incrementPointer

                            expected =
                                machine 10
                                    |> pointerAt 1
                        in
                        Expect.equal actual expected
                , test "decrementPointer" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> pointerAt 1
                                    |> decrementPointer

                            expected =
                                machine 10
                        in
                        Expect.equal actual expected
                , test "decrementPointer below zero" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> decrementPointer

                            expected =
                                machine 10
                        in
                        Expect.equal actual expected
                , test "incrementPointer above size" <|
                    \_ ->
                        let
                            actual =
                                machine 2
                                    |> incrementPointer
                                    |> incrementPointer
                                    |> incrementPointer

                            expected =
                                machine 2
                                    |> pointerAt 1
                        in
                        Expect.equal actual expected
                , test "increment" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> increment
                                    |> increment

                            expected =
                                machine 10
                                    |> valueAt 0 2
                        in
                        Expect.equal actual expected
                , test "decrement" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> increment
                                    |> decrement

                            expected =
                                machine 10
                        in
                        Expect.equal actual expected
                , test "decrement below zero" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> decrement

                            expected =
                                machine 10
                        in
                        Expect.equal actual expected
                , test "output" <|
                    \_ ->
                        let
                            actual =
                                machine 10
                                    |> valueAt 0 (Char.toCode 'A')
                                    |> output

                            expected =
                                actual
                                    |> withOutput "A"
                        in
                        Expect.equal actual expected
                ]
            ]
        ]
