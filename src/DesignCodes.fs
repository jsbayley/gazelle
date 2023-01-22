namespace Fortress

/// Supported eurocodes (EC).
type Eurocode = | UK

/// Supported global design codes.
type DesignCode = Eurocode of Eurocode
