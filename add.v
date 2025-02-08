module fa (
    input  wire i_a,
    input  wire i_b,
    input  wire i_cin,
    output wire o_sum,
    output wire o_cout
);
    assign o_sum  = i_a ^ i_b ^ i_cin;
    assign o_cout = (i_a & i_b) | (i_a & i_cin) | (i_b & i_cin);

    // tmp = i_a ^ i_b
    // o_sum = tmp ^ i_cin
    // o_cout = (tmp & i_cin) | (i_a & i_b)
endmodule

module adder (
    input  wire [3:0] i_op1,
    input  wire [3:0] i_op2,
    output wire [3:0] o_sum,
    output wire       o_cout
);
    wire [2:0] carry;
    fa fa [3:0] (.i_a(i_op1), .i_b(i_op2), .i_cin({carry, 1'b0}), .o_sum(o_sum), .o_cout({o_cout, carry}));
endmodule
