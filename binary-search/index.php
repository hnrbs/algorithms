<?php

$numbers = [5, 6, 7, 8, 9, 10, 11, 12, 13, 14];


function find(int $search, array $items) {
    $high = count($items) - 1;
    $low  = 0;

    return guess($high, $low, $items, $search);
}

function guess(int $high, int $low, array $items, int $search) {
    $mid = floor(($high + $low) / 2);

    $guess = $items[$mid];

    if ($low > $high) {
        die('the given item are not in the array');
    } 
    
    if ($guess == $search) {
        return $mid;
    }    

    if ($search < $items[$mid]) {
        $high = $mid - 1;
        return guess($high, $low, $items, $search);
    }

    $low = $mid + 1;
    return guess($high, $low, $items, $search);
}

$position = find(14, $numbers);

echo "Pos: {$position}";

