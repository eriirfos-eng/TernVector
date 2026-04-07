#!/usr/bin/env node

/**
 * Quick Demo - Edge-Net Simulation
 * Demonstrates key features with a fast, focused simulation
 */

import { NetworkSimulation } from '../src/network.js';

console.log(`
╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║              🚀  EDGE-NET QUICK DEMO  🚀                      ║
║                                                               ║
║   A 60-second tour of the network lifecycle simulation       ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
`);

async function runDemo() {
  console.log('\n📍 Phase 1: Genesis (0 - 10K nodes)\n');
  console.log('   Bootstrapping network with genesis nodes...');

  const sim = new NetworkSimulation({
    genesisNodes: 5,
    targetNodes: 15000, // Past genesis into transition
    tickInterval: 100,
    accelerationFactor: 50000,
  });

  await sim.initialize();

  // Show initial state
  console.log(`   ✓ ${sim.nodes.size} genesis nodes initialized`);
  console.log('   ✓ Genesis nodes interconnected');
  console.log('   ✓ 10x early adopter multiplier active\n');

  // Run through genesis
  let lastPhase = 'genesis';
  while (sim.nodes.size < 10000) {
    await sim.tick();

    if (Math.random() < 0.5) {
      sim.addNode();
    }

    if (sim.currentTick % 200 === 0) {
      const stats = Array.from(sim.nodes.values())[0].getStats();
      console.log(
        `   [${sim.currentTick}] Nodes: ${sim.nodes.size.toLocaleString()} | ` +
        `Genesis RFI-IRFOS: ${stats.ruvEarned.toLocaleString()}`
      );
    }
  }

  console.log('\n   ✅ Genesis phase complete!');
  console.log(`   • Network: ${sim.nodes.size.toLocaleString()} nodes`);
  console.log(`   • Compute: ${Math.floor(sim.totalComputeHours).toLocaleString()} hours`);
  console.log(`   • Health: ${(sim.metrics.networkHealth * 100).toFixed(1)}%\n`);

  console.log('\n📍 Phase 2: Transition (10K - 15K nodes)\n');
  console.log('   Genesis sunset preparation...');

  while (sim.nodes.size < 15000) {
    await sim.tick();

    if (Math.random() < 0.6) {
      sim.addNode();
    }

    const currentPhase = sim.getCurrentPhase();
    if (currentPhase !== lastPhase) {
      console.log(`\n   🔄 PHASE TRANSITION: ${lastPhase} → ${currentPhase}`);
      console.log('   • Genesis nodes limiting connections');
      console.log('   • Early multiplier decaying');
      console.log('   • Network resilience testing\n');
      lastPhase = currentPhase;
    }

    if (sim.currentTick % 200 === 0 && currentPhase === 'transition') {
      const genesisNode = Array.from(sim.nodes.values()).find(n => n.isGenesis);
      console.log(
        `   [${sim.currentTick}] Nodes: ${sim.nodes.size.toLocaleString()} | ` +
        `Genesis connections: ${genesisNode.maxConnections}`
      );
    }
  }

  console.log('\n   ✅ Transition phase reached!');
  console.log(`   • Network: ${sim.nodes.size.toLocaleString()} nodes`);
  console.log(`   • Tasks completed: ${sim.metrics.totalTasksCompleted.toLocaleString()}`);
  console.log(`   • Success rate: ${(sim.metrics.averageSuccessRate * 100).toFixed(2)}%\n`);

  // Final report
  const report = sim.generateReport();

  console.log('\n📊 DEMO RESULTS');
  console.log('─'.repeat(70));
  console.log(`
Network Metrics:
  • Total Nodes:           ${report.summary.totalNodes.toLocaleString()}
  • Active Nodes:          ${report.summary.activeNodes.toLocaleString()}
  • Genesis Nodes:         ${report.metrics.genesisNodeCount}
  • Total Compute:         ${Math.floor(report.summary.totalComputeHours).toLocaleString()} hours
  • Network Health:        ${(report.metrics.networkHealth * 100).toFixed(1)}%

Economic Summary:
  • Total RFI-IRFOS Supply:      ${report.economics.supply.total.toLocaleString()} RFI-IRFOS
  • Contributors Pool:     ${report.economics.supply.contributors.toLocaleString()} RFI-IRFOS (${((report.economics.supply.contributors / report.economics.supply.total) * 100).toFixed(1)}%)
  • Treasury:              ${report.economics.supply.treasury.toLocaleString()} RFI-IRFOS (${((report.economics.supply.treasury / report.economics.supply.total) * 100).toFixed(1)}%)
  • Protocol Fund:         ${report.economics.supply.protocol.toLocaleString()} RFI-IRFOS (${((report.economics.supply.protocol / report.economics.supply.total) * 100).toFixed(1)}%)
  • Economic Health:       ${(report.economics.health.overall * 100).toFixed(1)}%

Phase Transitions:
`);

  report.phases.transitions.forEach(t => {
    console.log(`  • ${t.from.padEnd(12)} → ${t.to.padEnd(12)} @ ${t.nodeCount.toLocaleString()} nodes`);
  });

  console.log(`
Top Genesis Contributors:
`);

  const topGenesis = report.nodes.genesis
    .sort((a, b) => b.ruvEarned - a.ruvEarned)
    .slice(0, 3);

  topGenesis.forEach((node, i) => {
    console.log(
      `  ${i + 1}. ${node.id.padEnd(10)} - ` +
      `${node.ruvEarned.toLocaleString().padStart(8)} RFI-IRFOS earned, ` +
      `${node.tasksCompleted.toLocaleString().padStart(5)} tasks completed`
    );
  });

  console.log('\n' + '─'.repeat(70));
  console.log('\n✅ Demo complete!');
  console.log('\nNext steps:');
  console.log('  • Run full simulation: npm run sim:full');
  console.log('  • Run tests: npm test');
  console.log('  • Generate visualizations: npm run visualize');
  console.log('  • Read documentation: cat README.md\n');
}

runDemo().catch(console.error);
