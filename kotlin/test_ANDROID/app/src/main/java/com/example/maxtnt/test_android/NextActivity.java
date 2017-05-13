package com.example.maxtnt.test_android;

import android.content.Intent;
import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.view.View;
import android.widget.Button;
import android.widget.Toast;

public class NextActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.next_activity_main);

        Button button_top = (Button) findViewById(R.id.topButton);
        button_top.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                Intent intent = new Intent(getApplicationContext(),ImageActivity.class);
                startActivity(intent);
            }
        });
    }
    public void click_back(View v){
        Toast.makeText(getApplicationContext(),"돌아가기",Toast.LENGTH_SHORT).show();
        Intent resultintent = new Intent();
        resultintent.putExtra("name","test");

        setResult(RESULT_OK,resultintent);
        finish();
    }

}