import 'package:flutter/material.dart';

class GENTWOIDE extends StatelessWidget {
    const GENTWOIDE({super.key});
    @override
    Widget build(BuildContext context) {
        /*IDE GUI*/
        return Scaffold(
            appBar: AppBar(
                title: const Text("GENTWO IDE"),
            ),
            body: const column(
                children: [
                    Text("GENTWO IDE"),
                ],
            ),
        )
    }
}