package com.nidjo123;

import java.util.List;

public abstract class Day implements Solution {
    protected List<String> lines;

    public void setInput(List<String> lines) {
        this.lines = lines;
    }
}
