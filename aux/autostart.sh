#!/bin/env bash
setxkbmap us -option caps:escape
xset r rate 200 50
$HOME/.config/polybar/launch.sh &
nm-applet &
picom &
exec feh --bg-scale ~/desktop.*
