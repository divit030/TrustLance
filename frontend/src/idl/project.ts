/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/project.json`.
 */
export type Project = {
  "address": "JAuKfWhmk78tNTNwiZLuKgNCHUZ9hYNQm5BiFouES3re",
  "metadata": {
    "name": "project",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "acceptBooking",
      "discriminator": [
        97,
        36,
        19,
        198,
        37,
        119,
        48,
        229
      ],
      "accounts": [
        {
          "name": "providerAuthority",
          "signer": true
        },
        {
          "name": "provider",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "providerAuthority"
              }
            ]
          }
        },
        {
          "name": "booking",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  111,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "booking.customer",
                "account": "booking"
              },
              {
                "kind": "arg",
                "path": "bookingId"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "bookingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "approveJob",
      "discriminator": [
        201,
        133,
        238,
        82,
        103,
        63,
        234,
        131
      ],
      "accounts": [
        {
          "name": "customerAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "customer",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  117,
                  115,
                  116,
                  111,
                  109,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              }
            ]
          }
        },
        {
          "name": "providerAuthority",
          "writable": true
        },
        {
          "name": "booking",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  111,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              },
              {
                "kind": "arg",
                "path": "bookingId"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "booking"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "bookingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelBooking",
      "discriminator": [
        139,
        162,
        116,
        202,
        78,
        140,
        139,
        90
      ],
      "accounts": [
        {
          "name": "customerAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "customer",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  117,
                  115,
                  116,
                  111,
                  109,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              }
            ]
          }
        },
        {
          "name": "booking",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  111,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              },
              {
                "kind": "arg",
                "path": "bookingId"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "booking"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "bookingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "completeJob",
      "discriminator": [
        221,
        216,
        225,
        72,
        101,
        250,
        3,
        11
      ],
      "accounts": [
        {
          "name": "providerAuthority",
          "signer": true
        },
        {
          "name": "provider",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "providerAuthority"
              }
            ]
          }
        },
        {
          "name": "booking",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  111,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "booking.customer",
                "account": "booking"
              },
              {
                "kind": "arg",
                "path": "bookingId"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "bookingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createBooking",
      "discriminator": [
        19,
        223,
        181,
        90,
        124,
        206,
        73,
        169
      ],
      "accounts": [
        {
          "name": "customerAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "customer",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  117,
                  115,
                  116,
                  111,
                  109,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              }
            ]
          }
        },
        {
          "name": "providerAuthority"
        },
        {
          "name": "provider",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "providerAuthority"
              }
            ]
          }
        },
        {
          "name": "booking",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  111,
                  107,
                  105,
                  110,
                  103
                ]
              },
              {
                "kind": "account",
                "path": "customerAuthority"
              },
              {
                "kind": "arg",
                "path": "bookingId"
              }
            ]
          }
        },
        {
          "name": "escrow",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "account",
                "path": "booking"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "bookingId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "counter",
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "registerCustomer",
      "discriminator": [
        211,
        203,
        193,
        164,
        198,
        246,
        27,
        223
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "customer",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  117,
                  115,
                  116,
                  111,
                  109,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "registerProvider",
      "discriminator": [
        254,
        209,
        54,
        184,
        46,
        197,
        109,
        78
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "provider",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  118,
                  105,
                  100,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "service",
          "type": "string"
        },
        {
          "name": "priceLamports",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "booking",
      "discriminator": [
        147,
        50,
        61,
        138,
        208,
        21,
        254,
        156
      ]
    },
    {
      "name": "counter",
      "discriminator": [
        255,
        176,
        4,
        245,
        188,
        253,
        124,
        25
      ]
    },
    {
      "name": "customer",
      "discriminator": [
        112,
        147,
        140,
        31,
        93,
        186,
        103,
        18
      ]
    },
    {
      "name": "escrow",
      "discriminator": [
        31,
        213,
        123,
        187,
        186,
        22,
        218,
        155
      ]
    },
    {
      "name": "provider",
      "discriminator": [
        164,
        180,
        71,
        17,
        75,
        216,
        80,
        195
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "customerNameEmpty",
      "msg": "Customer name cannot be empty."
    },
    {
      "code": 6001,
      "name": "customerNameTooLong",
      "msg": "Customer name cannot exceed 50 bytes."
    },
    {
      "code": 6002,
      "name": "customerNotRegistered",
      "msg": "Customer is not registered."
    },
    {
      "code": 6003,
      "name": "providerNotRegistered",
      "msg": "Provider is not registered."
    },
    {
      "code": 6004,
      "name": "invalidPrice",
      "msg": "Provider price must be greater than zero."
    },
    {
      "code": 6005,
      "name": "invalidBookingStatus",
      "msg": "This booking cannot be accepted in its current status."
    },
    {
      "code": 6006,
      "name": "unauthorizedCancellation",
      "msg": "Customer cannot cancel this booking."
    },
    {
      "code": 6007,
      "name": "invalidCompletionStatus",
      "msg": "Only accepted bookings can be marked as completed."
    },
    {
      "code": 6008,
      "name": "invalidApprovalStatus",
      "msg": "Only completed bookings can be approved."
    },
    {
      "code": 6009,
      "name": "invalidCancellationStatus",
      "msg": "Only newly created bookings can be cancelled."
    },
    {
      "code": 6010,
      "name": "insufficientEscrowFunds",
      "msg": "The escrow does not contain enough funds to release the payment."
    }
  ],
  "types": [
    {
      "name": "booking",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bookingId",
            "type": "u64"
          },
          {
            "name": "customer",
            "type": "pubkey"
          },
          {
            "name": "provider",
            "type": "pubkey"
          },
          {
            "name": "service",
            "type": "string"
          },
          {
            "name": "amountLamports",
            "type": "u64"
          },
          {
            "name": "status",
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "counter",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "count",
            "type": "u64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "customer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "registered",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "booking",
            "type": "pubkey"
          },
          {
            "name": "customer",
            "type": "pubkey"
          },
          {
            "name": "provider",
            "type": "pubkey"
          },
          {
            "name": "amountLamports",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "provider",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "service",
            "type": "string"
          },
          {
            "name": "priceLamports",
            "type": "u64"
          },
          {
            "name": "registered",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
