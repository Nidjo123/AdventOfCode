package com.nidjo123.days;

import com.nidjo123.Day;

import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class Day16 extends Day {
    private record Tunnel(Valve valve, int cost) {
    }

    private static class Valve {
        private final String name;
        private final int flow;
        private final List<Tunnel> connectedValves = new ArrayList<>();

        public Valve(String name, int flow) {
            this.name = name;
            this.flow = flow;
        }

        public int getFlow() {
            return flow;
        }

        public void connect(Valve other, int cost) {
            assert other != null;
            connectedValves.add(new Tunnel(other, cost));
        }

        public List<Tunnel> getConnectedValves() {
            return connectedValves;
        }

        public Valve cloneUnconnected() {
            return new Valve(name, flow);
        }
    }

    private final Map<String, Valve> valves = new HashMap<>();
    private final Pattern valveFlowPattern = Pattern.compile("Valve (?<name>\\w+) has flow rate=(?<flow>\\d+);");
    private final Pattern connectedValvesPattern = Pattern.compile("tunnels? leads? to valves? (?<valveList>(\\w+)(, \\w+)*)");

    private HashMap<Valve, Integer> getDistances(Valve valve) {
        record State(Valve valve, int steps) {
        }

        HashMap<Valve, Integer> distances = new HashMap<>();
        Set<Valve> visited = new HashSet<>();
        Queue<State> queue = new ArrayDeque<>();
        queue.add(new State(valve, 0));
        while (!queue.isEmpty()) {
            State state = queue.remove();
            if (visited.contains(state.valve)) {
                if (visited.size() == valves.size()) {
                    break;
                }
                continue;
            }
            visited.add(state.valve);
            distances.put(state.valve, state.steps);
            for (Tunnel tunnel : state.valve.getConnectedValves()) {
                queue.add(new State(tunnel.valve, state.steps + tunnel.cost));
            }
        }
        return distances;
    }

    private Map<Valve, Map<Valve, Integer>> computeMinDistances() {
        Map<Valve, Map<Valve, Integer>> minDistances = new HashMap<>();
        for (Valve valve : valves.values()) {
            minDistances.put(valve, getDistances(valve));
        }
        return minDistances;
    }

    private void rebuildGraph() {
        nonZeroFlowValves = valves.values().stream().filter(valve -> valve.flow > 0).collect(Collectors.toSet());
        Map<Valve, Map<Valve, Integer>> minDistances = computeMinDistances();
        Set<Valve> newValves = new HashSet<>();
        for (Valve valve : nonZeroFlowValves) {
            newValves.add(valve.cloneUnconnected());
        }
        for (Valve valve : newValves) {
            Valve originalValve = valves.get(valve.name);
            final Map<Valve, Integer> distances = minDistances.get(originalValve);
            for (Valve other : newValves) {
                if (valve != other) {
                    Valve originalOtherValve = valves.get(other.name);
                    int distance = distances.get(originalOtherValve);
                    valve.connect(other, distance);
                }
            }
        }
        Valve originalStartValve = valves.get("AA");
        Valve newStartValve = originalStartValve.cloneUnconnected();
        if (originalStartValve.getFlow() == 0) {
            Valve startValve = originalStartValve.cloneUnconnected();
            Map<Valve, Integer> fromStartValve = minDistances.get(originalStartValve);
            for (Valve other : newValves) {
                Valve originalOtherValve = valves.get(other.name);
                startValve.connect(other, fromStartValve.get(originalOtherValve));
                other.connect(startValve, minDistances.get(originalOtherValve).get(originalStartValve));
            }
            newStartValve = startValve;
        }
        valves.clear();
        valves.put(originalStartValve.name, newStartValve);
        for (Valve valve : newValves) {
            valves.put(valve.name, valve);
        }
        nonZeroFlowValves = valves.values().stream().filter(valve -> valve.flow > 0).collect(Collectors.toSet());
    }

    @Override
    public void init() {
        super.init();

        Map<String, String[]> connections = new HashMap<>();
        for (String line : lines) {
            if (line.isBlank()) {
                continue;
            }

            Matcher valveFlow = valveFlowPattern.matcher(line);
            if (valveFlow.find()) {
                String name = valveFlow.group("name");
                int flow = Integer.parseInt(valveFlow.group("flow"));
                valves.put(name, new Valve(name, flow));
                Matcher valveConnections = connectedValvesPattern.matcher(line);
                if (valveConnections.find()) {
                    String valves = valveConnections.group("valveList");
                    connections.put(name, valves.split(", "));
                }
            }
        }

        for (Map.Entry<String, String[]> entry : connections.entrySet()) {
            Valve from = valves.get(entry.getKey());
            for (String toName : entry.getValue()) {
                from.connect(valves.get(toName), 1);
            }
        }

        rebuildGraph();
    }

    private Set<Valve> nonZeroFlowValves = new HashSet<>();

    private int getAccumulatedPressure(Set<Valve> valves, int timeLeft) {
        int sum = 0;
        for (Valve valve : valves) {
            sum += valve.getFlow();
        }
        return sum * timeLeft;
    }

    private int maxFlowSoFar;

    private int calculateTheoreticalMaxFlow(Valve currentValve, Set<Valve> openValves, int acc, int timeLeft) {
        for (Tunnel tunnel : currentValve.getConnectedValves()) {
            if (tunnel.cost < timeLeft && !openValves.contains(tunnel.valve)) {
                acc += tunnel.valve.getFlow() * (timeLeft - tunnel.cost - 1);
            }
        }
        return acc;
    }

    private void updateMaxFlow(int flow) {
        if (flow > maxFlowSoFar) {
            maxFlowSoFar = flow;
        }
    }

    private int solve(Valve currentValve, Set<Valve> openValves, int acc, int timeLeft) {
        if (timeLeft == 0) {
            updateMaxFlow(acc);
            return acc;
        }
        final int toAcc = getAccumulatedPressure(openValves, timeLeft);
        final int accThisStep = toAcc / timeLeft;
        if (openValves.size() == nonZeroFlowValves.size()) {
            updateMaxFlow(acc + toAcc);
            return acc + toAcc;
        }
        int res = 0;
        if (currentValve.getFlow() > 0 && !openValves.contains(currentValve)) {
            openValves.add(currentValve);
            res = Math.max(res, solve(currentValve, openValves, acc + accThisStep, timeLeft - 1));
            openValves.remove(currentValve);
        }
        final int theoreticalMaxFlow = calculateTheoreticalMaxFlow(currentValve, openValves, acc + toAcc, timeLeft);
        if (theoreticalMaxFlow < maxFlowSoFar) {
            return res;
        }
        for (Tunnel tunnel : currentValve.getConnectedValves()) {
            if (tunnel.cost <= timeLeft) {
                res = Math.max(res, solve(tunnel.valve, openValves, acc + accThisStep * tunnel.cost, timeLeft - tunnel.cost));
            } else {
                res = Math.max(res, acc + toAcc);
            }
        }
        updateMaxFlow(res);
        return res;
    }

    @Override
    public void part1() {
        Valve startValve = valves.get("AA");
        int maxFlow = solve(startValve, new HashSet<>(), 0, 30);
        System.out.println(maxFlow);
    }

    @Override
    public void part2() {

    }
}
