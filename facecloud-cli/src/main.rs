use clap::{Parser, Subcommand};
use facecloud_core::neuromorphic::envelope::EnvelopeConfig;
use facecloud_core::neuromorphic::signals::{
    EmFieldIntensity, InflammationIndex, InterfaceCoherence, InterfaceTelemetry, MechDensity,
    SpikeEnergy, ThermalLoad,
};
use facecloud_core::safety::guard::GuardKernel;
use facecloud_dna_auth::mfa::{evaluate_mfa, DnaFactor, KnowledgeFactor, MultiLayerContext, PossessionFactor};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "facecloud-cli")]
#[command(about = "Facecloud safety and MFA inspector.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Envelope {
        mech_density: f32,
        interface_coherence: f32,
        em_field: f32,
        thermal: f32,
        inflammation: f32,
        spike: f32,
    },
    Mfa {
        #[arg(long)]
        knowledge: bool,
        #[arg(long)]
        possession: bool,
        #[arg(long)]
        dna_confidence: Option<f32>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Envelope {
            mech_density,
            interface_coherence,
            em_field,
            thermal,
            inflammation,
            spike,
        } => {
            let telemetry = InterfaceTelemetry {
                mech_density: MechDensity(mech_density),
                interface_coherence: InterfaceCoherence(interface_coherence),
                em_field: EmFieldIntensity(em_field),
                thermal_load: ThermalLoad(thermal),
                inflammation: InflammationIndex(inflammation),
                spike_energy: SpikeEnergy(spike),
            };
            let kernel = GuardKernel {
                config: EnvelopeConfig::default(),
            };
            let rec = kernel.evaluate(&telemetry);
            println!("{}", serde_json::to_string_pretty(&rec).unwrap());
        }
        Commands::Mfa {
            knowledge,
            possession,
            dna_confidence,
        } => {
            let ctx = MultiLayerContext {
                knowledge: KnowledgeFactor { present: knowledge },
                possession: PossessionFactor {
                    present: possession,
                },
                dna: dna_confidence.map(|c| DnaFactor {
                    id: Uuid::new_v4(),
                    hash_reference: "dna-ref-placeholder".to_string(),
                    confidence: c,
                }),
            };
            let eval = evaluate_mfa(&ctx);
            println!("{}", serde_json::to_string_pretty(&eval).unwrap());
        }
    }
}
