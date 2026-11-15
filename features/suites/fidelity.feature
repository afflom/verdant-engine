Feature: fidelity

  How deeply Verdant Engine's backend model states what it claims, and how the
  permanent zero-constants parametric ratchet prevents magic literals,
  ungrounded modules, or Classical.choice from ever entering the codebase.

  @FI-01 @build
  Scenario: Every record of a Prism project is scored on W3C PROV-O derivation, ISO/IEC/IEEE 29148 verification, and ISO/IEC/IEEE 15939 measurement, refusing falsified digests
    Given a Prism project with a record derived from a source span, one named by a theorem, a quality measure with a measurement result, and one with none of these
    When prism-fidelity scores it
    Then each record's questions are closed or open exactly as its rows and theorems say
    And a source entity whose kappa does not recompute is refused

  @FI-02 @build
  Scenario: The fidelity register only shrinks: a closed kappa never reopens, and committed fidelity.toml and Fidelity.lex.tex match the live model
    Given a register in which a record's derived question is closed
    When the record's derivation is removed and the project is scored again
    Then the next register is refused, naming the record and the axis
    And the committed fidelity.toml and app/src/Fidelity.lex.tex equal what prism-fidelity writes for app/

  @FI-03 @build
  Scenario: Verdant Engine's own model fidelity is measured by ISO/IEC/IEEE 15939 indicators checked by the Lean kernel
    Given app/src/Fidelity.lex.tex written by prism-fidelity from app/
    When the Lean kernel checks Fidelity::indicatorsKnown and Fidelity::everyIndicatorMeasured
    Then there is one indicator per property whose value is the register's open count on that axis
    And every result's kappa recomputes from fidelity.toml

  @FI-04 @build
  Scenario: Records are cited mechanically from the smallest verbatim span in model/provenance.toml primary sources
    Given a project whose stakeholder text is stated in charter.md and whose component text is stated nowhere
    When prism-fidelity cites it from the sources model/provenance.toml lists
    Then the stakeholder is derived from the one line of charter.md that states it
    And app/src/Provenance.lex.tex equals what prism-fidelity cites for app/

  @FI-05 @build
  Scenario: Every quality measure listed in model/measures.toml is evaluated by the Lean kernel against its declared threshold
    Given model/measures.toml and the constructive methods in app/src/Measures.lex.tex
    When prism-fidelity writes app/src/Fidelity.lex.tex and the Lean kernel checks it
    Then each listed measure has an indicator bounded by its threshold and a decided theorem

  @FI-06 @build
  Scenario: Record-to-record W3C PROV-O derivations close only when their upstream records close and refuse unreferenced derivations
    Given a record that references a record cited from a source
    When prism-fidelity cites the project and scores it
    Then the record is derived from the record it references and unreferenced derivations are refused

  @FI-07 @build
  Scenario: Primary charter sources are attributed to their human author as a W3C PROV-O person agent
    Given model/provenance.toml names the author of model/charter.md
    When prism-fidelity cites the project
    Then every source entity of the charter is attributed to that person agent

  @FI-08 @build
  Scenario: The permanent zero-constants parametric ratchet enforces zero magic float or integer literals across verdant-core and constructive LexLean modules and refuses planted constants
    Given model/parametric-ratchet.toml with all six ceilings locked at zero
    When audit_parametric_ratchet audits Verdant Engine and planted-defect scratch copies
    Then the live repository passes with zero magic constants and zero Classical.choice
    And planted magic float/int literals in Rust, literals > 1 in constructive LexLean, Classical.choice, ungrounded modules, or non-zero ceilings are refused
