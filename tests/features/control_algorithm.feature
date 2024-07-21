# Add reasonable tests for your control algorithm here
#
# - The control algorithm should prevent the tanks from overflowing.
# - A reasonable simulation time for these tests are anything between 30 and 300 seconds

Feature: Control algorithm for tanks of an airplane

  Scenario: Prevent left tank from overflowing
    Given the left tank is at 150 liter
    And the right tank is at 100 liter
    When 100 seconds pass
    Then the left tank should not overflow
    And the right tank should not overflow

  Scenario: Prevent right tank from overflowing
    Given the left tank is at 100 liter
    And the right tank is at 200 liter
    When 100 seconds pass
    Then the left tank should not overflow
    And the right tank should not overflow

  Scenario: Prevent both tanks from overflowing with high initial levels
    Given the left tank is at 180 liter
    And the right tank is at 230 liter
    When 150 seconds pass
    Then the left tank should not overflow
    And the right tank should not overflow

  Scenario: Handle balanced tank levels
    Given the left tank is at 100 liter
    And the right tank is at 100 liter
    When 200 seconds pass
    Then the left tank should not overflow
    And the right tank should not overflow
